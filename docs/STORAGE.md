# SQLite storage implementation v1

storage::Store uses one local SQLite file for multiple documents. Chat, evidence, graph rules and assessments are future schema additions. This is a library/CLI implementation, not a local web server.

## Public operations

- open(path): initialize an empty database or verify its application/schema version.
- create_document(title): create an empty document and initial revision atomically.
- document_ids(): enumerate stored document identities.
- snapshot(document): load the current committed version in a consistent read transaction.
- revision(document, revision): load a historical version without changing the current head.
- apply(document, command): validate and atomically persist a supported operation batch.

create_document is not yet an idempotent service command. Historical loading is not a restore command. Provider/user authorization remains the future service layer's responsibility.

## Tables and space use

Migration 001 creates documents, revisions, claim_versions and receipts with foreign keys and strict SQL column types. Application ID 0x4c535243 and user_version 1 identify this experimental format. An unrelated nonempty database or a future version is refused, not overwritten. This migration initializes empty workspaces; it does not upgrade other formats.

Revision manifests store section/block layout, exact claim references, heads and review markers. Claim bodies are stored once in claim_versions and reconstructed for a historical revision using their introducing revision's sequence. An unchanged document does not create a new revision.

Accepted proposal payloads are also retained in receipts to preserve revision reasons and requested operations. This duplicates changed content in its originating operation, but does not duplicate every unchanged claim at every checkpoint. Receipts store SHA-256 request fingerprints and exact outcomes, including generated IDs.

Manifests, claim versions and receipts are currently retained indefinitely. Dependency-aware retention and cleanup remain open; this implementation reduces repeated text but does not promise bounded disk usage. Current manifests still repeat layout/reference metadata. Do not manually prune claim-introducing revisions: historical reconstruction depends on them.

## Transactions and retries

apply takes an immediate write transaction before reading the head. It checks for an existing workspace-wide idempotency key, loads committed state and runs the core validator. It writes the revision, new claim bodies, head and receipt before committing. A final-step failure rolls back the entire transaction. No authoritative in-memory cache survives a failed commit.

Request fingerprints include the document ID, expected revision and complete parsed proposal. The same key/input returns the original outcome even after restart or a later head change. A key reused with different input fails. Failed transactions reserve neither keys nor candidate IDs; retries may generate new IDs because none were acknowledged.

Concurrent connections serialize writes and check expected versions. Stale clients are rejected. Busy operations wait up to five seconds and then surface an SQLite error. This is database-level protection, not the planned single-service lock.

## Durability and validation limits

Foreign keys are enabled on each connection. The initial profile uses rollback journaling with synchronous=EXTRA; WAL-mode files are currently refused. Reads decode typed manifests and validate identities, structural uniqueness and claim references before reconstructing the engine.

Tests cover reopen, historical claim text, retries, stale clients, late transaction failure, SQLITE_FULL through a page limit, and a child process exiting during an uncommitted transaction. The ignored interrupted_writer test is a helper invoked by the recovery test, not an omitted acceptance test.

Tests do not simulate every filesystem/device power-loss behavior. Arbitrarily hostile databases, malicious schema triggers and a full security audit remain outside this increment. Validation is not cryptographic authentication. The database contains private learning content, is not encrypted and must not be committed. API keys are not stored here.

## Remaining work

- Consistent backup/restore and portable asset manifests; do not copy a live database as a backup procedure.
- Dependency-aware pruning, storage accounting and physical space reclamation.
- Publication/index jobs and generated Markdown files.
- Full relational claim/evidence/graph schemas and audit actor metadata.
- Versioned migrations with backup/rollback tooling beyond initial creation.
- Single-service locking and browser-facing authorization.

Bundled SQLite requires a C compiler at build time; future packaged binaries will not. Linux validation is local; Windows CI results must be checked independently.
