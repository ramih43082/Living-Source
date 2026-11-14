# Current revision and retention direction

See [F08](FEATURES.md#f08--claim-versioning-provenance-and-revision-impact), [F09](FEATURES.md#f09--revision-history-storage-growth-and-recovery) and [F22](FEATURES.md#f22--persistence-portability-and-backups).

## Accepted behavior

The viewer is read-only. Chat/graph changes eventually produce validated atomic operations. Changed claim content creates a new version; old evidence and scenarios remain tied to old versions. Reject stale writes rather than overriding newer changes.

Current code preserves historical document states and claim bodies and supports idempotent operation replay after restart. Reading history is implemented; restoring, diffing and undo interfaces are not.

## Bloat and recovery

Do not retain every token/keystroke as a permanent snapshot. Keep recoverable drafts separate from commits. Share unchanged content, retain meaningful checkpoints and define thinning for unreferenced intermediate history. Protect the entire dependency closure of evidence/assessments/scenarios.

Exact retention periods, checkpoint defaults and backups remain owner decisions. Current storage retains history/receipts indefinitely, so bounded growth is not implemented.

## Future transitions

A restore should create a new head referencing a historical state without rolling back independent assessments/scenarios. External JSON edits become validated candidates, not silent overwrites. Backup/restore must consistently include referenced assets and preserve identities.

Generated Markdown publication is no longer on the current path. Any future JSON file export needs a specified consistency/import policy, but the application must not create two uncontrolled writable authorities.

Read [STORAGE](STORAGE.md) for tested transaction behavior. The [older lifecycle draft](archive/DOCUMENT_REVISIONS-pre-json.md) is historical only.
