//! Local persistence: state, provenance and replay receipts commit together.
use crate::{Command, Engine, Error, Outcome, Snapshot};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path, time::Duration};
use uuid::Uuid;

const APPLICATION_ID: i32 = 0x4c535243;
const SCHEMA_VERSION: i32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Engine(#[from] Error),
    #[error("SQLite operation failed: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("stored data cannot be decoded: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("unsupported schema or unrelated database; no migration applied")]
    IncompatibleDatabase,
    #[error("inconsistent stored document")]
    CorruptDocument,
}

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let mut connection = Connection::open(path)?;
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "synchronous", "EXTRA")?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let app: i32 = tx.pragma_query_value(None, "application_id", |r| r.get(0))?;
        let version: i32 = tx.pragma_query_value(None, "user_version", |r| r.get(0))?;
        let objects: i64 = tx.query_row(
            "SELECT count(*) FROM sqlite_schema WHERE name NOT LIKE 'sqlite_%'",
            [],
            |r| r.get(0),
        )?;
        if app == 0 && version == 0 && objects == 0 {
            tx.execute_batch(include_str!("../migrations/001_initial.sql"))?;
            tx.pragma_update(None, "application_id", APPLICATION_ID)?;
            tx.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        } else if app != APPLICATION_ID || version != SCHEMA_VERSION {
            return Err(StorageError::IncompatibleDatabase);
        }
        tx.commit()?;
        let mode: String = connection.pragma_query_value(None, "journal_mode", |r| r.get(0))?;
        if mode != "delete" && mode != "memory" {
            return Err(StorageError::IncompatibleDatabase);
        }
        Ok(Self { connection })
    }

    pub fn create_document(&mut self, title: String) -> Result<Snapshot, StorageError> {
        let state = Engine::new(title)?.state;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO documents(id, head_revision) VALUES (?1, ?2)",
            params![state.document_id.to_string(), state.revision_id.to_string()],
        )?;
        write_revision(&tx, &state, 0, None, 0)?;
        tx.commit()?;
        Ok(state)
    }

    pub fn document_ids(&self) -> Result<Vec<Uuid>, StorageError> {
        let mut stmt = self
            .connection
            .prepare("SELECT id FROM documents ORDER BY id")?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        rows.map(|r| Uuid::parse_str(&r?).map_err(|_| StorageError::CorruptDocument))
            .collect()
    }

    pub fn snapshot(&mut self, document: Uuid) -> Result<Snapshot, StorageError> {
        let tx = self.connection.transaction()?;
        let head = head(&tx, document)?;
        let result = read_revision(&tx, document, &head)?.0;
        tx.commit()?;
        Ok(result)
    }

    pub fn revision(&mut self, document: Uuid, revision: Uuid) -> Result<Snapshot, StorageError> {
        let tx = self.connection.transaction()?;
        let result = read_revision(&tx, document, &revision.to_string())?.0;
        tx.commit()?;
        Ok(result)
    }

    pub fn apply(&mut self, document: Uuid, command: Command) -> Result<Outcome, StorageError> {
        crate::validate_proposal(&command.proposal)?;
        let bytes = serde_json::to_vec(&(document, command.expected_revision, &command.proposal))?;
        if bytes.len() > crate::MAX_PROPOSAL_BYTES {
            return Err(Error::LimitExceeded.into());
        }
        let hash = Sha256::digest(&bytes).to_vec();
        let key = command.idempotency_key.to_string();
        let proposal_json = serde_json::to_string(&command.proposal)?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let previous: Option<(Vec<u8>, String)> = tx
            .query_row(
                "SELECT request_hash, outcome FROM receipts WHERE key = ?1",
                [&key],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        if let Some((old_hash, outcome)) = previous {
            if old_hash != hash {
                return Err(Error::IdempotencyConflict.into());
            }
            return Ok(serde_json::from_str(&outcome)?);
        }
        let old_head = head(&tx, document)?;
        let (state, sequence) = read_revision(&tx, document, &old_head)?;
        let old_claim_count = state.claim_versions.len();
        let mut engine = Engine {
            state,
            receipts: BTreeMap::new(),
        };
        let outcome = engine.apply(command)?;
        if outcome.revision_id.to_string() != old_head {
            write_revision(
                &tx,
                &engine.state,
                sequence + 1,
                Some(&old_head),
                old_claim_count,
            )?;
            tx.execute(
                "UPDATE documents SET head_revision = ?1 WHERE id = ?2",
                params![outcome.revision_id.to_string(), document.to_string()],
            )?;
        }
        tx.execute("INSERT INTO receipts(key, document_id, revision_id, request_hash, proposal, outcome) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![key, document.to_string(), outcome.revision_id.to_string(), hash, proposal_json, serde_json::to_string(&outcome)?])?;
        tx.commit()?;
        Ok(outcome)
    }
}

fn head(db: &Connection, document: Uuid) -> Result<String, StorageError> {
    db.query_row(
        "SELECT head_revision FROM documents WHERE id = ?1",
        [document.to_string()],
        |r| r.get(0),
    )
    .optional()?
    .ok_or_else(|| Error::UnknownReference.into())
}

fn write_revision(
    db: &Connection,
    state: &Snapshot,
    sequence: i64,
    parent: Option<&str>,
    old_count: usize,
) -> Result<(), StorageError> {
    let mut manifest = state.clone();
    manifest.claim_versions.clear();
    db.execute("INSERT INTO revisions(document_id, id, sequence, parent_id, manifest) VALUES (?1, ?2, ?3, ?4, ?5)", params![state.document_id.to_string(), state.revision_id.to_string(), sequence, parent, serde_json::to_string(&manifest)?])?;
    for (ordinal, claim) in state.claim_versions.iter().enumerate().skip(old_count) {
        db.execute("INSERT INTO claim_versions(document_id, version_id, introduced_revision, ordinal, payload) VALUES (?1, ?2, ?3, ?4, ?5)", params![state.document_id.to_string(), claim.reference.version_id.to_string(), state.revision_id.to_string(), ordinal as i64, serde_json::to_string(claim)?])?;
    }
    Ok(())
}

fn read_revision(
    db: &Connection,
    document: Uuid,
    revision: &str,
) -> Result<(Snapshot, i64), StorageError> {
    let (manifest, sequence): (String, i64) = db
        .query_row(
            "SELECT manifest, sequence FROM revisions WHERE document_id = ?1 AND id = ?2",
            params![document.to_string(), revision],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?
        .ok_or(Error::UnknownReference)?;
    let mut state: Snapshot = serde_json::from_str(&manifest)?;
    if state.document_id != document
        || state.revision_id.to_string() != revision
        || state.format_version != 1
        || !state.claim_versions.is_empty()
    {
        return Err(StorageError::CorruptDocument);
    }
    let mut stmt = db.prepare("SELECT c.payload FROM claim_versions c JOIN revisions r ON c.document_id = r.document_id AND c.introduced_revision = r.id WHERE c.document_id = ?1 AND r.sequence <= ?2 ORDER BY c.ordinal")?;
    let rows = stmt.query_map(params![document.to_string(), sequence], |r| {
        r.get::<_, String>(0)
    })?;
    for row in rows {
        state.claim_versions.push(serde_json::from_str(&row?)?);
    }
    validate_stored(&state)?;
    Ok((state, sequence))
}

fn validate_stored(s: &Snapshot) -> Result<(), StorageError> {
    crate::nonempty(&s.title)?;
    let mut ids = std::collections::BTreeSet::new();
    let mut versions = std::collections::BTreeSet::new();
    for c in &s.claim_versions {
        crate::content(&c.content)?;
        if c.reference.owner_document_id != s.document_id
            || !versions.insert(c.reference.version_id)
        {
            return Err(StorageError::CorruptDocument);
        }
    }
    for (item, version) in &s.claim_heads {
        crate::find_claim(
            s,
            &crate::ClaimRef {
                owner_document_id: s.document_id,
                item_id: *item,
                version_id: *version,
            },
        )?;
    }
    for claim in &s.claim_versions {
        if !s.claim_heads.contains_key(&claim.reference.item_id) {
            return Err(StorageError::CorruptDocument);
        }
    }
    for section in &s.sections {
        crate::nonempty(&section.title)?;
        if !ids.insert(section.id) {
            return Err(StorageError::CorruptDocument);
        }
        for block in &section.blocks {
            if !ids.insert(block.id) {
                return Err(StorageError::CorruptDocument);
            }
            crate::find_claim(s, &block.claim)?;
        }
    }
    for reference in &s.review_required {
        crate::find_claim(s, reference)?;
    }
    if ids.len() > crate::MAX_ITEMS || s.claim_versions.len() > crate::MAX_ITEMS {
        return Err(Error::LimitExceeded.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sqlite_full_does_not_partially_save() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("full.sqlite");
        let mut db = Store::open(&path).unwrap();
        let original = db.create_document("Keep".into()).unwrap();
        let pages: i64 = db
            .connection
            .pragma_query_value(None, "page_count", |r| r.get(0))
            .unwrap();
        db.connection
            .pragma_update(None, "max_page_count", pages)
            .unwrap();
        let proposal = crate::parse_proposal(&serde_json::to_vec(&serde_json::json!({"protocol_version":1,"explanation":"allocate","operations":[{
            "op":"create_claim","op_id":"1","new_handle":"new:c","content":{"statement_markdown":"x".repeat(60_000),"language":"en","scope_markdown":"","assumptions":[],"interpretation_markdown":""}
        }]})).unwrap()).unwrap();
        let error = db
            .apply(
                original.document_id,
                Command {
                    expected_revision: original.revision_id,
                    idempotency_key: Uuid::new_v4(),
                    proposal,
                },
            )
            .unwrap_err();
        assert!(
            matches!(error, StorageError::Sqlite(ref e) if e.sqlite_error_code() == Some(rusqlite::ErrorCode::DiskFull))
        );
        drop(db);
        let mut reopened = Store::open(&path).unwrap();
        assert_eq!(reopened.snapshot(original.document_id).unwrap(), original);
        assert_eq!(
            reopened
                .connection
                .query_row("SELECT count(*) FROM receipts", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            0
        );
    }
}
