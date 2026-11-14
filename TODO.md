# Work register

Current instruction: detailed handoff for incoming contributors; implementation may resume later. [FEATURE_STATUS](docs/FEATURE_STATUS.md) is the status authority and [DECISIONS](docs/DECISIONS.md) is the decision queue.

## Documentation completed in this handoff

- [x] Register viewer-first scope and JSON-only document format.
- [x] Document detailed feature behavior, examples, boundaries and optional ideas.
- [x] Separate accepted decisions from recommendations and unanswered choices.
- [x] Link feature status to code/tests and identify prototype-only behavior.
- [x] Preserve superseded technical drafts as historical references.
- [x] Provide a contributor reading order and proposed continuation sequence.

## Existing implementation

- [x] Rust workspace, pinned toolchain, Cargo.lock and CLI examples.
- [x] Six-operation core with strict payload parsing, stable IDs, atomic batches and version checks.
- [x] Generated schemas for the experimental subset.
- [x] SQLite initial schema, document/claim history and durable operation retry records.
- [x] Recorded 19 local tests passing; fmt/Clippy passed during implementation.
- [x] Linux/Windows CI configured; remote/packaging outcomes not confirmed.

## First-viewer work, only when coding resumes

- [ ] Resolve JSON loading workflow, initial layout and representative fixtures.
- [ ] Finalize Section/Explanation/Claim/Reasoning schema and versioning.
- [ ] Implement safe read-only rendering, math, citations and navigation.
- [ ] Validate malformed inputs, Unicode/RTL, long content and accessibility.
- [ ] Connect SQLite if required by the chosen viewer workflow.
- [ ] Agree acceptance/release criteria with Rami.

## Later product work

- [ ] Full typed graph/chat operations, review diffs and service authorization.
- [ ] Local server/session protections and platform packaging.
- [ ] DeepSeek adapter, conversation storage, usage/cancellation/model provenance.
- [ ] Credential store, key lifecycle and security audit.
- [ ] Source ingestion, exact evidence/citations and hybrid local RAG.
- [ ] Curation, contextual assessments, challenges and dependency review.
- [ ] Scenario evaluation, traces, conflict containment and comparison.
- [ ] Bounded history, safe cleanup, consistent backup/restore and OS migration.
- [ ] JSON import/export with dependency preservation.
- [ ] Optional learning aids, formal checking, OCR, publication export and external API/MCP.

No current task requires MyST, Logseq integration, custom Markdown file syntax, direct document editing, a separate exam mode or cloud synchronization.
