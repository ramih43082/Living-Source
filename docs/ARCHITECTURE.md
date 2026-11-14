# Current architecture direction

The accepted product is described in [FEATURES](FEATURES.md), and actual implementation in [FEATURE_STATUS](FEATURE_STATUS.md).

## Boundaries

- Local single-user browser application, intended for Linux and Windows 11.
- Rust backend; reusable TypeScript frontend is the existing technical direction.
- JSON document model with Markdown/LaTeX content fields.
- One SQLite workspace database for durable state/history; imported assets and derived indexes can live alongside it.
- Read-only document viewer, with future chat/graph operations using the same validated backend.
- Future local retrieval plus external AI generation; local storage does not mean offline inference.

The first milestone is the viewer. The full chat-left/document-and-graph-right arrangement remains the later workspace target.

## Authority and flow

Trusted application command context + proposed operations → schema/reference/version/authorization checks → atomic durable state → read-only rendering.

JSON describes document structure; SQLite persists it. A portable export must be versioned and validated on import. No independently writable Markdown mirror or MyST parser is currently required.

## Existing versus future

Existing: core operations, immutable claim versions, SQLite initial schema, transactional saves, historical reads, durable retry receipts and tests.

Future: local server/session boundary, viewer, complete content types, safe rendering, graph actions, AI adapter, OS credential store, source ingestion, retrieval, assessment/scenario engines, backup/retention, packaging and updates.

Exact frameworks, local authentication, import limits, asset layout and backup implementation must be evaluated rather than guessed. Do not claim localhost alone is secure or that copying a live database is a consistent backup.

See [STORAGE](STORAGE.md) for the actual storage profile and [the historical architecture](archive/ARCHITECTURE-pre-json.md) only for prior rationale.
