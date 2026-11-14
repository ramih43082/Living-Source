# Current data-model direction

The code implements only a subset of the long-term model. [FEATURE_STATUS](FEATURE_STATUS.md) and [STORAGE](STORAGE.md) are authoritative about what exists.

## Existing

Documents and current revision pointers; root sections and claim display blocks; claim content and immutable versions; review-required markers; accepted proposal/outcome receipts; initial schema metadata.

SQLite tables are documents, revisions, claim_versions and receipts. Revision manifests retain layout/references while claim bodies are stored separately. Read [migration 001](../crates/document-core/migrations/001_initial.sql) and [Rust types](../crates/document-core/src/lib.rs) before changing persistence.

## Target conceptual entities

| Group | Required information |
| --- | --- |
| Document | Identity, schema/revision, ordered Section/Explanation/Claim/Reasoning content, language framing and scope |
| Knowledge | Claim/rule/relationship identities and versions, domain/assumptions/interpretation |
| Evidence | Original asset, edition/translation, extraction version, exact passage/locator and citation |
| Curation/assessment | Inclusion independent from reviewer/method/evidence/scope judgments |
| Discussion | Document conversation, messages, AI run/proposal, exact context and model provenance |
| Scenario | Pinned assumptions/rules, interpretation, evaluation configuration, signed results/traces |
| Operations | Expected versions, authorization context, idempotency, atomic results, review/derived jobs |
| Workspace | Schema migrations, portable assets, backup metadata, retention/dependency references |

Entity IDs differ from version IDs. Evidence/assessments/scenarios pin versions; navigation may separately offer newer content. Do not merge claims by embedding similarity or identical wording.

JSON is the document contract; database normalization is an implementation choice. No requirement to create separate SQLite databases for chat, document and graph exists. Secrets are excluded from document and workspace exports.

The [historical detailed model](archive/DATA_MODEL-pre-json.md) contains useful candidate entities, but its Markdown-authority and earlier lifecycle proposals are superseded. Translate the current viewer model into a reviewed schema before extending migrations.
