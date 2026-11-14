# Document revision lifecycle — updated design draft

> HISTORICAL DRAFT — superseded. Preserved for rationale only. In particular, Markdown file authority, direct editing and the earlier release sequence are obsolete. Read [current decisions](../DECISIONS.md), [features](../FEATURES.md) and [the current version of this note](../DOCUMENT_REVISIONS.md).


Date: 14 November 2026. Supersedes direct document editing and whole-Markdown AI replacement. The pane is read-only; [typed operations](../EDIT_OPERATIONS.md) from chat/graph are the only in-app mutation path. Design only; not fault-tested implementation.

## Commit lifecycle

1. Read the committed structured document and its generated Markdown.
2. Gather a graph form submission or chat-generated operation proposal pinned to the current document/item versions.
3. Persist recoverable form/proposal state separately; streaming text is not a document revision.
4. Validate against [the schema](../DOCUMENT_SCHEMA.md), resolve handles and build a candidate.
5. Display an impact diff when required; preserve precise learner authorization.
6. In one SQLite transaction recheck versions, create changed object versions, advance document head, record idempotent outcome and enqueue publication/index/review jobs.
7. Acknowledge committed state separately from Markdown publication status.
8. Deterministically publish the portable file and refresh derived indexes asynchronously.

Full serialized document replacement is not an editing command. IDs, claim versions, citations and dependencies are assigned/validated by the service. Move/reorder preserves identity. Changed claim content creates a new version and explicit display-selection operations; old references remain unchanged.

## Concurrency and restore

If a graph action or another chat run commits after an AI run starts, hold that AI proposal for reconciliation. Check versions again after review. No silent force-save or automatic rebase.

Restoring an older document creates a new head from that historical structure, records its origin and preserves old history. It does not roll back global assessments or change saved scenarios. Preview the resulting version references; archived dependencies must still resolve.

## Storage and bounded history

Use one SQLite database for authoritative typed records, chat, immutable object versions and checkpoint metadata. Markdown is a generated portable representation. A database transaction does not atomically publish a separate file, so publication jobs are durable/retryable.

Separate recovery persistence, current state and long-term history:
- Chat input/graph forms/proposals use replaceable recovery drafts, debounced with a maximum persistence interval during continuous input. No per-token/per-keystroke permanent snapshots.
- Accepted operation batches create versioned commits; an unchanged operation is a no-op.
- Meaningful retained checkpoints include applied AI batches, explicit checkpoints and restores.
- Unreferenced routine document history may be thinned under a visible retention policy; exact counts/ages are still open.
- Revisions pinned by citations, assessments, curated knowledge, retained proposals/traces or scenarios cannot be removed while referenced. Pruning operates on the full dependency closure, not just direct references.
- Content-addressed immutable block/knowledge versions let unchanged content be shared across checkpoints. Full generated Markdown can be cached for current/pinned revisions rather than copied on every draft update.
- Historical evidence locators include immutable content and serializer version; materialize/pin exact bytes before dropping any representation required by a citation.
- Provide storage accounting and a preview for cleanup. Do not promise physical file shrinkage from logical deletion alone; space reclamation is a separate tested maintenance step.

Implement current state/recovery correctly first. Storage representation, retention defaults and compaction require measured fixtures; do not introduce complex delta chains merely to reduce estimated size.

## File publication and external changes

The backend publishes documents/<document-id>.md using a deterministic serializer/version, a sibling temporary file and platform-tested replacement/flush behavior. Titles do not define file paths.

Track expected file hashes. Unexpected external edits are captured as import candidates and block automatic replacement. Parse and validate them, translate accepted differences into typed proposals, then review through chat/graph. Imported claims of verification never create trusted assessments.

Missing/moved files do not delete live documents. Duplicate IDs, malformed structure or unsupported formats are explicit errors. Preserve observed original bytes for recovery. External editors do not follow application locks, so hash checks cannot guarantee capture of every simultaneous external write; do not promise that guarantee.

## Failure recovery and backup

| Failure | Outcome |
| --- | --- |
| Partial/failed AI response | Keep committed state; preserve useful proposal/error information. |
| Transaction failure or disk full | No success acknowledgement or partial batch. |
| Commit succeeds before file publication | Recover from database and retry durable job. |
| File published before job acknowledgement | Compare expected hash and acknowledge without another revision. |
| Index interrupted | Rebuild against exact versions; expose stale coverage. |
| External file changed | Preserve candidate; require reconciliation, never silently trust file metadata. |

Serialize workspace writes in one service, while multiple tabs still use version checks. Backups capture a consistent supported SQLite snapshot, referenced immutable assets and a manifest of schema/serializer versions and hashes. Include any retained external candidates; exclude credentials. Derived indexes and generated current Markdown can be rebuilt.

Archive by default. Permanent erasure is separate from unreferenced history cleanup; neither should cascade through cited evidence silently.

## Required future checks

Test stale graph/chat operations, idempotent retries, partial streaming, disk-full and interrupted publication, exact citation replay, restoration without assessment rollback, Unicode/math preservation, referenced-history protection, export round trips and Linux-to-Windows restoration. These remain unchecked implementation work.
