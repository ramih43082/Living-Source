# Local architecture

## Target

Single-user local website accessed through a browser on Linux or Windows 11. A Rust executable serves the interface and API on loopback only. Reuse the TypeScript interface; it need not be rewritten in Rust.

Cloud storage and synchronization are not required. DeepSeek is the initial external inference service. Embedding-model installation may require a download; indexing should operate locally thereafter.

The [data model](DATA_MODEL.md) and [revision lifecycle](DOCUMENT_REVISIONS.md) refine this architecture as proposed designs. They are not implemented migrations.

## Components

| Component | Responsibility |
| --- | --- |
| Browser UI | Chat, Markdown/math editor, graph, evidence and review views, themes |
| Rust service | Workspace operations, ingestion, context assembly, retrieval, provider calls |
| Local persistence | Markdown files, original references, SQLite records and revision snapshots |
| Search | Full-text and vector retrieval, ranking, deduplication, graph expansion |
| Scenario engine | Explicit multi-premise rules, signed derivations, conflict containment |
| Credential adapter | OS secret-store access; stored keys are never returned to frontend code |
| Provider adapter | DeepSeek requests, validation, errors, timeouts, usage and model metadata |

Select exact frameworks and dependencies after portability, license and runtime checks.

## Proposed workspace layout

- `workspace.json`: workspace identity and schema version; no secrets.
- `documents/`: portable Markdown working copies of committed documents.
- `references/`: imported originals.
- `database.sqlite`: authoritative committed Markdown snapshots, conversations, stable IDs, claims, relations, assessments and jobs.
- Revision snapshots are initially kept in SQLite; a separate revisions directory is not required.
- `indexes/`: rebuildable search data.
- `exports/`: user-requested exports.

Use relative paths and stable IDs. Document/reference revisions are user data; search indexes are derived.

Proposed consistency strategy: commit Markdown snapshots, metadata, head pointers and publication jobs in one SQLite transaction. Publish ordinary Markdown copies afterward through retryable jobs. Distinguish committed content from pending file publication. A database transaction alone does not atomically update a separate Markdown file. External changes are preserved as candidates and reconciled; see the revision lifecycle for limits of concurrent external-editor writes.

Detect external Markdown edits and reconcile them without losing anchors or overwriting pending AI work.

## Minimum entities

Workspace/schema migration; learning document/version; stable block anchor; reference/edition/translation/version; extracted passage/locator/quality; conversation/message; claim/version/curation; assessment; typed relationship; multi-premise rule; challenge; scenario/version/trace; retrieval run/evidence bundle; ingestion/index job.

Authorship, curation, assessment and hypothetical acceptance are independent.

## API responsibilities

Typed operations for documents with expected-version checks; discussion; AI proposals; manual promotion/assessment; reference import and source scope; search and citation inspection; scenario evaluation; workspace backup/restore; credential set/check/remove.

A future MCP adapter can expose compatible operations to Codex or Claude Code without bypassing manual curation. This is optional.

## Security boundaries — audit pending

Localhost alone is not authentication. Audit:
- Loopback binding and unexpected LAN exposure.
- Origin/Host validation, DNS rebinding, CSRF and local sessions.
- Authorization for state-changing operations.
- File path traversal, arbitrary file access and unsafe imports.
- Markdown/math rendering and XSS.
- Source/agent prompt injection.
- Secrets in memory, logs, errors, exports and crash reports.
- Provider destinations, request sizes, resource limits and cancellation.
- Dependency and update provenance.

Use an OS credential-store adapter. If a Linux secret service is missing or locked, report it clearly and offer explicit session-only handling rather than silently storing plaintext. Support replacement and deletion.

Never commit or include credentials in workspace backups or exported documents.

## Portability and backups

Build separate Linux and Windows 11 executables serving the same interface. Backups must combine a consistent SQLite snapshot with corresponding file revisions, preserving IDs, paths and citation/assessment links.

Do not recommend copying a live SQLite database without handling WAL and consistency. Restore tests must include cross-OS migration and missing reference files.

Changing OS means installing the appropriate executable, restoring the workspace and entering the API key again. Credential-store contents are excluded from backups.

## Migration sequence

1. Preserve accepted interface behavior.
2. Replace hosted API routes with Rust loopback endpoints.
3. Replace browser persistence with workspace operations.
4. Import prototype JSON while preserving unsupported legacy fields.
5. Add OS credential storage and local-session protections.
6. Add retrieval and references incrementally.
7. Test backup/restore and offline document editing on both OSs.

Moving this plan to GitHub does not automatically migrate prototype code or personal browser data.
