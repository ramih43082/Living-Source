# Document revision lifecycle — design draft

Status: proposed, 14 November 2026. Defines behavior and recovery boundaries; not yet implemented or fault-tested. Complements the [data model](DATA_MODEL.md).

## Storage decision

Use immutable complete Markdown snapshots in SQLite as the authoritative committed document history. Keep ordinary Markdown files under documents/ as portable working copies. The database also holds graph metadata and revision links.

This deliberately duplicates current Markdown content. It makes document content, anchors, citations, revision pointer and job scheduling commit together in one database transaction. File publication happens afterward and is recoverable. Do not attempt to claim one atomic transaction across SQLite and independent Markdown files.

The tradeoff is additional storage and the need to expose file-export failures clearly. Initial full snapshots favor simplicity over delta compression; measure storage before changing that design. A plain Markdown export remains readable independently, but full workspace backup is needed for conversations, provenance and assessments.

## Lifecycle

1. Open: load current revision plus any recoverable draft. Show which is committed and which is unsaved.
2. Edit: user changes update a recoverable draft tied to a base revision. Debounce draft persistence; expose saving/failure state. Do not create a permanent revision for every keystroke.
3. Commit: on explicit save or a debounced autosave, submit full candidate Markdown, base revision ID and a unique operation key.
4. Validate: syntax/rendering limits, payload size, anchor/citation structure and base ownership. Markdown need not contain assessed claims to be saved.
5. In one SQLite transaction: compare expected head, insert immutable snapshot/occurrences/citations, advance head, record operation result and enqueue publication/index-impact work. Roll back all on failure.
6. Acknowledge committed: return the new revision ID. Show a separate “Markdown file pending” state if publication has not completed.
7. Publish: export the committed snapshot to its stable ID-based filename; display titles need not be valid Windows filenames.
8. Index asynchronously: stale/missing index state never prevents document editing. Retrieval identifies the exact indexed revision.

Check the head inside the transaction, not only before it. Repeated requests with the same operation key return the original outcome. No-op content/title saves can return the existing revision without creating history.

Draft recovery is best-effort up to the last acknowledged draft save; unsent keystrokes cannot be guaranteed after a browser crash. Define debounce values and limits during implementation, with visible failure behavior.

## AI edits and user edits

An AI run pins the starting document revision and relevant evidence. Streamed text is a draft/proposal, not an incremental overwrite of committed Markdown. Cancellation or provider errors leave the existing document intact.

| Situation | Behavior |
| --- | --- |
| AI completes; head still equals base | Validate and apply if automatic document updates are enabled; otherwise show a proposed diff. |
| Learner or another tab saved since AI started | Hold proposal for review, even if changes appear disjoint. Show base/current/proposed comparison. |
| Learner accepts selected changes | Build a reconciled candidate against the current head; commit with a fresh head check. |
| Head changes again during review | Refresh the comparison; never force-overwrite. |
| Invalid or partial AI output | Retain the proposal/error for inspection; do not update the head. |
| User rejects proposal | Record rejection; leave document and assessments unchanged. |

Proposal states: generating, ready, needs_review, applied, rejected, cancelled or failed. A ready proposal becomes needs_review when its base is stale. Applying it and recording the applied revision happen in the same transaction.

An AI-generated block may enter the working document without entering the curated source. Add to source is a separate user action against a pinned statement/version; verification is another explicit action.

## Anchors, citations and meaning

A block ID denotes continuity of a paragraph, heading, equation or other structural unit; it does not assert that its meaning is unchanged.

- Structured edits preserve block IDs where the operation establishes continuity; inserted blocks get new IDs.
- Raw Markdown edits use a deterministic structural diff. Preserve IDs for unambiguous unchanged/moved blocks; ambiguous matches require new IDs and retain mapping candidates.
- Splits/merges create new IDs with explicit lineage. Never choose arbitrarily between identical repeated paragraphs.
- Store offsets as UTF-8 byte ranges into the immutable snapshot, with parser/version metadata. Browser adapters convert their string indices; do not mix UTF-16 offsets with byte offsets.
- Every saved citation resolves to the original version/span. Mapping to new text is an additional navigation aid, not a replacement.
- Claims extracted from edited text are proposals. Keep existing claims/assessments on their old versions and open impact-review tasks where needed.
- Unchanged claims explicitly referenced from a revised document can retain their own existing versions. That does not verify the entire new document.

Example: revision 12 states a theorem for positive integers and is linked to a reviewed claim. Revision 13 changes the domain to all integers. The old assessment still describes the old claim; the broadened statement needs a new claim version and review. A scenario using the old claim remains reproducible.

## External Markdown edits

In v1 the stable documents/<document-id>.md path identifies the working copy; renaming a file is not automatic document deletion or identity reassignment.

Track the last exported revision/hash. When a file changes:
1. Capture its bytes to a durable recovery candidate before overwriting anything.
2. If database head still equals the last exported revision, offer/import it as an external edit based on that revision, with the usual head check.
3. If both database and file changed, preserve both and require reconciliation.
4. For ambiguous anchor matches, create new identities and show unresolved mappings.
5. Missing files are reported; moving/deleting a working copy does not delete committed history.

V1 uses conservative review for external imports. A malformed/unsupported file is retained as a candidate and reported, not discarded.

Publication uses a sibling temporary file and platform-tested replace/flush behavior. Before publishing, check the working file against the last-export hash; unexpected bytes are preserved and publication is blocked pending reconciliation. External editors do not obey application locks: hash checks alone cannot eliminate a last-moment race. Do not claim lossless simultaneous arbitrary external writes. Detect as much as possible, retain observed candidates and advise closing external editors during reconciliation. Stronger filesystem coordination remains a platform implementation question.

## Recovery and backup

| Failure point | Required outcome |
| --- | --- |
| Before database commit | Old head remains; recover last persisted draft/proposal if available. |
| Database commit fails or disk is full | No success acknowledgement; preserve recoverable input and show failure. |
| Database committed; Markdown not published | Committed revision remains authoritative; retry durable publication job on restart. |
| File replaced; job acknowledgement interrupted | Compare target hash, acknowledge if already correct; retry without creating a new revision. |
| Index build interrupted | Resume/rebuild against pinned version; label stale coverage. |
| Unexpected external file detected | Preserve candidate and block replacement; no silent overwrite. |

The service serializes workspace mutations and rejects a second writer service. Multiple browser tabs still use expected-revision checks. Short-lived backup/maintenance locks are separate from normal editing.

Backups capture a consistent SQLite snapshot and every immutable original asset it references. Block destructive asset cleanup while taking the backup; use a supported SQLite backup procedure rather than copying an open database file. Include a manifest of hashes and schema version. Current Markdown copies can be regenerated from committed snapshots. Capture any external edits through reconciliation first or include them explicitly as uncommitted recovery candidates.

Restore verifies assets and integrity before activating the workspace; missing evidence is reported. Credentials are excluded. Test actual restore on Linux and Windows before claiming portability.

## Undo, restore and deletion

Undo in an unsaved editor affects the draft. Restoring a committed historical revision creates a new revision whose parent is the current head and whose provenance records the restored revision. Current citations/assessments are never silently rewritten.

Archiving hides a document from ordinary navigation while preserving citations and history. Permanent deletion, retention pruning and history compression are deferred designs; do not implement cascading erasure as a convenient default.

## Required acceptance checks

- Save, restart and recover exact Markdown including multilingual text and equations.
- Two tabs start at revision A; one commits B; the other's A-based save is held for reconciliation.
- AI starts at A, user commits B, AI finishes: B remains until an explicit reconciliation commit.
- Retry the same save after lost response: exactly one committed revision.
- Interrupt publication after a database commit: recover the committed document and retry export.
- Inject disk-full/write failures: no false success or lost previously committed revision.
- Restore an older revision: history and old citations survive.
- Move/split/duplicate paragraphs: no silently misassigned anchors.
- Change a theorem's assumptions: no inherited verification.
- Reconcile an externally edited file against both unchanged and changed database heads.
- Back up and restore across OSs with all evidence/assessment links intact and no credentials.
