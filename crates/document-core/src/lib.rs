//! Document engine and SQLite persistence. Authorization belongs to the service layer.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

pub mod storage;

pub const MAX_PROPOSAL_BYTES: usize = 1024 * 1024;
const MAX_FIELD_BYTES: usize = 64 * 1024;
const MAX_OPERATIONS: usize = 100;
const MAX_ITEMS: usize = 10_000;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("invalid proposal: {0}")]
    InvalidShape(String),
    #[error("unsupported protocol version")]
    UnsupportedVersion,
    #[error("resource limit exceeded")]
    LimitExceeded,
    #[error("stale document or claim version")]
    StaleVersion,
    #[error("unknown reference")]
    UnknownReference,
    #[error("invalid structure: {0}")]
    InvalidStructure(String),
    #[error("idempotency key reused with different input")]
    IdempotencyConflict,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ClaimContent {
    pub statement_markdown: String,
    pub language: String,
    pub scope_markdown: String,
    pub assumptions: Vec<Assumption>,
    pub interpretation_markdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Assumption {
    pub label: String,
    pub text_markdown: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ClaimRef {
    pub owner_document_id: Uuid,
    pub item_id: Uuid,
    pub version_id: Uuid,
}

/// Explicit tags avoid ambiguous model-generated references.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ClaimTarget {
    Existing { reference: ClaimRef },
    Local { handle: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SectionTarget {
    Existing { id: Uuid },
    Local { handle: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Proposal {
    pub protocol_version: u32,
    pub explanation: String,
    pub operations: Vec<Operation>,
}

/// First supported vertical slice. Unsupported planned operations fail closed.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    RenameDocument {
        op_id: String,
        title: String,
    },
    AddSection {
        op_id: String,
        new_handle: String,
        title: String,
    },
    CreateClaim {
        op_id: String,
        new_handle: String,
        content: ClaimContent,
    },
    PlaceClaim {
        op_id: String,
        new_block_handle: String,
        section: SectionTarget,
        claim: ClaimTarget,
    },
    ReviseClaim {
        op_id: String,
        claim_ref: ClaimRef,
        new_version_handle: String,
        content: ClaimContent,
        reason: String,
    },
    SelectClaimVersion {
        op_id: String,
        block_id: Uuid,
        expected_claim_ref: ClaimRef,
        replacement: ClaimTarget,
    },
}

impl Operation {
    fn id(&self) -> &str {
        match self {
            Self::RenameDocument { op_id, .. }
            | Self::AddSection { op_id, .. }
            | Self::CreateClaim { op_id, .. }
            | Self::PlaceClaim { op_id, .. }
            | Self::ReviseClaim { op_id, .. }
            | Self::SelectClaimVersion { op_id, .. } => op_id,
        }
    }
}

/// Produced by the trusted caller, never deserialized from an LLM response.
pub struct Command {
    pub expected_revision: Uuid,
    pub idempotency_key: Uuid,
    pub proposal: Proposal,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ClaimVersion {
    pub reference: ClaimRef,
    pub content: ClaimContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ClaimBlock {
    pub id: Uuid,
    pub claim: ClaimRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub id: Uuid,
    pub title: String,
    pub blocks: Vec<ClaimBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub format_version: u32,
    pub document_id: Uuid,
    pub revision_id: Uuid,
    pub title: String,
    pub sections: Vec<Section>,
    pub claim_versions: Vec<ClaimVersion>,
    pub claim_heads: BTreeMap<Uuid, Uuid>,
    /// Changed old versions whose downstream uses need review.
    pub review_required: Vec<ClaimRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub revision_id: Uuid,
    pub created_ids: BTreeMap<String, Uuid>,
    pub created_claim_refs: BTreeMap<String, ClaimRef>,
}

/// In-memory engine; use storage::Store for durable transactions and replay.
pub struct Engine {
    state: Snapshot,
    receipts: BTreeMap<Uuid, (Vec<u8>, Outcome)>,
}

fn field(s: &str) -> Result<(), Error> {
    if s.len() > MAX_FIELD_BYTES {
        Err(Error::LimitExceeded)
    } else {
        Ok(())
    }
}

fn nonempty(s: &str) -> Result<(), Error> {
    field(s)?;
    if s.trim().is_empty() {
        Err(Error::InvalidShape("empty value".into()))
    } else {
        Ok(())
    }
}

fn content(c: &ClaimContent) -> Result<(), Error> {
    nonempty(&c.statement_markdown)?;
    nonempty(&c.language)?;
    field(&c.scope_markdown)?;
    field(&c.interpretation_markdown)?;
    if c.assumptions.len() > MAX_OPERATIONS {
        return Err(Error::LimitExceeded);
    }
    let mut labels = BTreeSet::new();
    for a in &c.assumptions {
        nonempty(&a.label)?;
        nonempty(&a.text_markdown)?;
        if !labels.insert(&a.label) {
            return Err(Error::InvalidStructure("duplicate assumption label".into()));
        }
    }
    Ok(())
}

pub fn parse_proposal(bytes: &[u8]) -> Result<Proposal, Error> {
    if bytes.len() > MAX_PROPOSAL_BYTES {
        return Err(Error::LimitExceeded);
    }
    let p: Proposal =
        serde_json::from_slice(bytes).map_err(|e| Error::InvalidShape(e.to_string()))?;
    validate_proposal(&p)?;
    Ok(p)
}

fn validate_proposal(p: &Proposal) -> Result<(), Error> {
    if p.protocol_version != 1 {
        return Err(Error::UnsupportedVersion);
    }
    if p.operations.is_empty() || p.operations.len() > MAX_OPERATIONS {
        return Err(Error::LimitExceeded);
    }
    field(&p.explanation)?;
    let mut ids = BTreeSet::new();
    for op in &p.operations {
        nonempty(op.id())?;
        if !ids.insert(op.id()) {
            return Err(Error::InvalidStructure("duplicate operation ID".into()));
        }
        match op {
            Operation::RenameDocument { title, .. } | Operation::AddSection { title, .. } => {
                nonempty(title)?
            }
            Operation::CreateClaim { content: c, .. } => content(c)?,
            Operation::ReviseClaim {
                content: c, reason, ..
            } => {
                content(c)?;
                nonempty(reason)?;
            }
            _ => (),
        }
    }
    Ok(())
}

fn find_claim<'a>(s: &'a Snapshot, r: &ClaimRef) -> Result<&'a ClaimVersion, Error> {
    s.claim_versions
        .iter()
        .find(|c| c.reference == *r)
        .ok_or(Error::UnknownReference)
}

fn resolve_claim(
    s: &Snapshot,
    t: &ClaimTarget,
    locals: &BTreeMap<String, ClaimRef>,
) -> Result<ClaimRef, Error> {
    let r = match t {
        ClaimTarget::Existing { reference } => reference.clone(),
        ClaimTarget::Local { handle } => {
            locals.get(handle).cloned().ok_or(Error::UnknownReference)?
        }
    };
    find_claim(s, &r)?;
    Ok(r)
}

fn register(handle: &str, ids: &mut BTreeMap<String, Uuid>, id: Uuid) -> Result<(), Error> {
    if !handle.starts_with("new:") || handle.len() <= 4 || handle.len() > 128 {
        return Err(Error::InvalidShape("invalid local handle".into()));
    }
    if ids.insert(handle.into(), id).is_some() {
        return Err(Error::InvalidStructure("duplicate local handle".into()));
    }
    Ok(())
}

impl Engine {
    pub fn new(title: String) -> Result<Self, Error> {
        nonempty(&title)?;
        Ok(Self {
            state: Snapshot {
                format_version: 1,
                document_id: Uuid::new_v4(),
                revision_id: Uuid::new_v4(),
                title,
                sections: vec![],
                claim_versions: vec![],
                claim_heads: BTreeMap::new(),
                review_required: vec![],
            },
            receipts: BTreeMap::new(),
        })
    }

    pub fn snapshot(&self) -> &Snapshot {
        &self.state
    }

    pub fn apply(&mut self, command: Command) -> Result<Outcome, Error> {
        validate_proposal(&command.proposal)?;
        let fingerprint = serde_json::to_vec(&(command.expected_revision, &command.proposal))
            .map_err(|e| Error::InvalidShape(e.to_string()))?;
        if fingerprint.len() > MAX_PROPOSAL_BYTES {
            return Err(Error::LimitExceeded);
        }
        if let Some((previous, outcome)) = self.receipts.get(&command.idempotency_key) {
            return if *previous == fingerprint {
                Ok(outcome.clone())
            } else {
                Err(Error::IdempotencyConflict)
            };
        }
        if self.state.revision_id != command.expected_revision {
            return Err(Error::StaleVersion);
        }
        let mut candidate = self.state.clone();
        let mut ids = BTreeMap::new();
        let mut refs = BTreeMap::new();
        for op in command.proposal.operations {
            match op {
                Operation::RenameDocument { title, .. } => candidate.title = title,
                Operation::AddSection {
                    new_handle, title, ..
                } => {
                    let id = Uuid::new_v4();
                    register(&new_handle, &mut ids, id)?;
                    candidate.sections.push(Section {
                        id,
                        title,
                        blocks: vec![],
                    });
                }
                Operation::CreateClaim {
                    new_handle,
                    content,
                    ..
                } => {
                    let r = ClaimRef {
                        owner_document_id: candidate.document_id,
                        item_id: Uuid::new_v4(),
                        version_id: Uuid::new_v4(),
                    };
                    register(&new_handle, &mut ids, r.item_id)?;
                    refs.insert(new_handle, r.clone());
                    candidate.claim_heads.insert(r.item_id, r.version_id);
                    candidate.claim_versions.push(ClaimVersion {
                        reference: r,
                        content,
                    });
                }
                Operation::PlaceClaim {
                    new_block_handle,
                    section,
                    claim,
                    ..
                } => {
                    let r = resolve_claim(&candidate, &claim, &refs)?;
                    let section_id = match section {
                        SectionTarget::Existing { id } => id,
                        SectionTarget::Local { handle } => {
                            *ids.get(&handle).ok_or(Error::UnknownReference)?
                        }
                    };
                    let target = candidate
                        .sections
                        .iter_mut()
                        .find(|s| s.id == section_id)
                        .ok_or(Error::UnknownReference)?;
                    let id = Uuid::new_v4();
                    register(&new_block_handle, &mut ids, id)?;
                    target.blocks.push(ClaimBlock { id, claim: r });
                }
                Operation::ReviseClaim {
                    claim_ref,
                    new_version_handle,
                    content,
                    ..
                } => {
                    find_claim(&candidate, &claim_ref)?;
                    if candidate.claim_heads.get(&claim_ref.item_id) != Some(&claim_ref.version_id)
                    {
                        return Err(Error::StaleVersion);
                    }
                    let r = ClaimRef {
                        version_id: Uuid::new_v4(),
                        ..claim_ref.clone()
                    };
                    register(&new_version_handle, &mut ids, r.version_id)?;
                    refs.insert(new_version_handle, r.clone());
                    candidate.claim_heads.insert(r.item_id, r.version_id);
                    candidate.review_required.push(claim_ref);
                    candidate.claim_versions.push(ClaimVersion {
                        reference: r,
                        content,
                    });
                }
                Operation::SelectClaimVersion {
                    block_id,
                    expected_claim_ref,
                    replacement,
                    ..
                } => {
                    let r = resolve_claim(&candidate, &replacement, &refs)?;
                    if r.item_id != expected_claim_ref.item_id {
                        return Err(Error::InvalidStructure(
                            "selection must retain claim identity".into(),
                        ));
                    }
                    let block = candidate
                        .sections
                        .iter_mut()
                        .flat_map(|s| &mut s.blocks)
                        .find(|b| b.id == block_id)
                        .ok_or(Error::UnknownReference)?;
                    if block.claim != expected_claim_ref {
                        return Err(Error::StaleVersion);
                    }
                    block.claim = r;
                }
            }
        }
        if candidate.sections.len()
            + candidate
                .sections
                .iter()
                .map(|s| s.blocks.len())
                .sum::<usize>()
            > MAX_ITEMS
            || candidate.claim_versions.len() > MAX_ITEMS
        {
            return Err(Error::LimitExceeded);
        }
        if candidate != self.state {
            candidate.revision_id = Uuid::new_v4();
        }
        let outcome = Outcome {
            revision_id: candidate.revision_id,
            created_ids: ids,
            created_claim_refs: refs,
        };
        self.state = candidate;
        self.receipts
            .insert(command.idempotency_key, (fingerprint, outcome.clone()));
        Ok(outcome)
    }
}
