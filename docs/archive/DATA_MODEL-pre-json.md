# Data model — design draft

> HISTORICAL DRAFT — superseded. Preserved for rationale only. In particular, Markdown file authority, direct editing and the earlier release sequence are obsolete. Read [current decisions](../DECISIONS.md), [features](../FEATURES.md) and [the current version of this note](../DATA_MODEL.md).


Status: proposed implementation design, 14 November 2026. Not an implemented database or a finalized SQL migration. This refines the product requirements without changing them. Read with [document revisions](../DOCUMENT_REVISIONS.md), [document schema](../DOCUMENT_SCHEMA.md) and [operation contract](../EDIT_OPERATIONS.md). These newer contracts supersede the earlier freeform editing/Markdown snapshot design.

## Core choice: identity is different from version

A document or claim has a permanent ID. Its content lives in immutable versions. Links that mean “what was used as evidence” always identify a version; navigation may separately offer the latest version.

Use application-generated opaque UUIDs stored as text initially, UTC timestamps, and SHA-256 content hashes. IDs are identity, hashes are integrity/deduplication aids, and timestamps are not ordering or concurrency controls. Never merge two claims merely because their text or embeddings match.

One SQLite database belongs to one workspace. Enable foreign-key enforcement on every connection. Relations use explicit foreign keys, not unchecked generic object IDs. Same-owner constraints require composite foreign keys or transactional validation. JSON is reserved for versioned domain-specific payloads, not core relationships.

## Durable core

The following are logical records and key fields, not literal SQL. Each version records its creator actor and creation time. Human, AI and import/system actors are distinguishable.

| Record | Fields and purpose |
| --- | --- |
| workspace | ID, schema version, creation time. Database metadata is authoritative; workspace.json is a discoverable manifest. |
| actor | ID, kind, display name. AI provenance points to a run; no credentials. |
| document | ID, current revision ID, archive time. Current title is read from its revision. |
| document_revision | ID, document ID, parent revision ID, title, immutable structured manifest referencing block/knowledge versions, serializer version, content hash, actor, origin, operation ID, restored-from revision if applicable. Immutable snapshots. |
| block | ID, document ID. Stable structural identity, never a truth assessment. |
| block_occurrence | Revision ID + block ID/version, parent section, ordinal, kind, generated UTF-8 byte start/end, text hash. At most one occurrence of a block per revision; offsets address that exact snapshot. |
| anchor_mapping | Old/new occurrences, mapping kind (same, changed, split, merged), reviewer/method. Ambiguous mappings remain unresolved. |
| conversation | ID, owning document ID, title, archive time. Multiple threads per document are allowed. |
| message | ID, conversation ID, sequence, role, content, status, reply/replacement ID, optional AI run ID. Final messages are immutable; edits create replacements. |
| edit_proposal | ID, document ID, base revision, typed operation batch/hash and trusted expected-version context, AI run/message, lifecycle state, applied revision if any. Candidate content is immutable; a revised candidate is a new proposal. |
| draft | ID, document ID, base revision, chat/graph session, recoverable form/proposal buffer, update sequence. Recoverable working state, never cited as committed evidence. |
| operation | Unique idempotency key, command kind, request hash, resulting revision/outcome. Reusing a key with different input fails. |
| projection_job | Document ID, target revision, expected last-export hash, status/error. Retryable Markdown file publication. |

Document revision chains are linear in v1. A restored revision becomes a new child of the current head; it does not rewind history. A manually reconciled AI edit records the original proposal/base and current parent for provenance without introducing branch-merging machinery.

Sections, typed block versions, immutable claim/rule display references and dependency manifests follow DOCUMENT_SCHEMA. Snapshot manifests share unchanged immutable versions. Generated Markdown is a deterministic representation; pin exact serialized bytes for historical evidence when needed. One service owns both structured commits and file generation, not two independent writable stores.

## Reference materials and exact evidence

| Record | Fields and purpose |
| --- | --- |
| reference | ID and stable identity of one bibliographic item/edition/translation. Different editions/translations are distinct items, optionally grouped by work. |
| reference_version | ID, reference ID, original asset hash/path, immutable bibliographic metadata and language. A changed import creates a version. |
| asset | Hash, workspace-relative location, media type, byte size, availability. Originals are immutable. |
| extraction | ID, reference version, extractor/config version, status, coverage/quality notes. Re-extraction creates a new record. |
| passage | ID, extraction ID, exact text/hash, structural parent, ordinal and original locator. PDF file page and printed page label are separate. |
| evidence_span | ID, exactly one document revision or extracted passage, byte range, exact quote/hash, optional block ID. Foreign keys and an exclusive-target check enforce the target. |
| citation | ID, citing document revision and range or message, evidence span, use (quote/paraphrase/etc.). A citing-target exclusive check is required. |
| source_scope_revision | ID, document ID, explicit included document/reference IDs and version-selection policy. Retrieval resolves latest policies to exact versions per run. |

Import copies originals into managed storage in v1. Externally linked reference files are deferred; external changes to exported learning Markdown are quarantined as import candidates and require validated reconciliation. Do not deduplicate bibliographic identities simply because original files share a hash.

An evidence span never silently retargets to current text. Current navigation may show a reviewed mapping alongside the original citation. A missing original produces an explicit unavailable indicator, not a fabricated reconstructed quotation.

## Curated knowledge and assessments

Use a shared knowledge_item identity with kind claim, relationship or rule. Each knowledge_version belongs to an item and has exactly one matching subtype below. This provides a real foreign-key target for assessments; subtype/owner consistency must be enforced.

| Record | Fields and purpose |
| --- | --- |
| claim_version | Knowledge version ID, statement, language, domain/context, assumptions, interpretation/speaker/genre and optional formal expression with schema version. |
| relationship_version | Knowledge version ID, typed source/target claim-version IDs, scope and explanation. Support/citation/interpretation are not inference. |
| rule_version | Knowledge version ID, scope/conditions, formalism identifier/version, explanation. |
| rule_premise | Rule version, position, claim version, polarity. Every listed premise is required. |
| rule_conclusion | Rule version, claim version, polarity. One conclusion per rule initially. |
| knowledge_origin | Knowledge version, evidence span or message, attribution type and derivation explanation. Keep author assertion distinct from our reconstruction. |
| curation_event | Knowledge version, actor, include/withdraw action, time, rationale and destination collection. Only an explicit learner action promotes. |
| assessment | Knowledge version, reviewer, judgment, method, scope, rationale, time and optional superseded assessment ID. Append-only. |
| assessment_evidence | Assessment ID, evidence span or knowledge version, role. Typed exclusive references. |
| challenge | ID, exact target knowledge version, challenge claim version, kind (counterexample, proof gap, contrary evidence), explanation. The challenge itself can be assessed. |
| review_task | Target version, triggering change/challenge, reason, open/resolved state and resolution record. Workflow state, not truth status. |

No assessment means unverified. Store multiple scoped assessments; do not implement “latest timestamp wins” as truth. Withdrawals/supersessions retain history. A changed knowledge version does not inherit verification or curation automatically. Linking unchanged existing claim versions from new prose is allowed and is different from verifying the revised prose.

A document may contain many claims and a claim may be expressed in several documents. Source value/reliability notes are annotations, not blanket truth judgments. A disproved premise flags affected derivations; it does not automatically refute every conclusion.

## Scenario and AI provenance records

- scenario / scenario_revision: identity and immutable premise/rule selection, interpretation and evaluation settings.
- scenario_assumption: scenario revision, claim version and polarity.
- scenario_rule: scenario revision, rule version and explicit admission provenance; distinguish reviewed from conditionally admitted provisional rules.
- claim_set_revision / member: reviewed extraction of a book or selection, exact claim versions and coverage. Scenario acceptance materializes membership, not a live “all claims” query.
- scenario_run: pinned scenario revision, engine/formalism version, limits, completion state.
- derivation / derivation_premise: signed conclusion, rule or assumption origin, concrete supporting derivations. Positive and negative traces coexist. Truncated evaluation is distinct from unresolved.
- ai_run: provider, requested/returned model, prompt-template version, source-scope revision, base document revision, usage, status/error and timestamps. No secrets or raw authorization headers.
- retrieval_run / retrieval_hit: query, model/index versions, budgets, exact evidence spans, rank and lineage.
- ai_context_item: ordered snapshot references to messages, revisions and evidence actually sent, plus transformations/truncation. Store the assembled payload locally if needed for replay; never credentials.
- index_job / index_generation: exact source/extraction/model versions, status and failure. Search indexes and embeddings are disposable; evidence/provenance are durable.

Inference eligibility is separate from curation and assessment: an unverified rule may be explicitly admitted in a hypothetical scenario without becoming verified. A conditional result promoted to knowledge retains its scenario revision and trace, not merely its final sentence.

## Invariants, deletion and migrations

1. Evidence, assessments and scenario inputs pin immutable versions.
2. Only document heads, drafts, workflow/job states and similar operational pointers mutate in place.
3. Scope, meaning and assumptions are part of claim content. A wording change may be substantive; similarity cannot decide equivalence.
4. Foreign keys restrict deletion of referenced history. Archive by default. Unreferenced routine history can be thinned under an explicit retention policy after checking the complete dependency closure; drafts are replaceable, not permanent snapshots. Permanent erasure requires a future explicit policy and must report broken/deleted evidence honestly; backups may retain prior data.
5. Source revisions/extractions and index generations are separate; stale search results cannot impersonate current evidence.
6. Schema migrations have numbered versions, prerequisites, integrity checks and a recoverable backup. No destructive migration is justified by the current design draft.
7. Secrets live outside these records; connection settings contain only a credential-store reference.

## Implementation slices

First implement workspace, actor, document/revision, block occurrences, drafts, operations and projection jobs. Add conversations/AI proposals next, then references/evidence/retrieval. Curated knowledge and scenarios build on these same identities later.

Before coding migrations, resolve exact SQL types/checks, schema-versioned payloads, indexes, limits and migration fixtures. The restricted inference language and formal proof semantics remain a separate design task.
