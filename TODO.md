# Roadmap and TODO

Open work for the target local application. Prototype features are not evidence that these migration tasks are complete.

## Implemented foundation

- [x] Rust workspace, pinned toolchain and Cargo.lock.
- [x] Six-operation in-memory document/claim engine with atomic batches and stale-version checks.
- [x] Generated schemas, example fixture and CLI demo.
- [x] Nineteen core/storage tests passing on development Linux; formatting and Clippy checks pass.
- [x] SQLite initial migration, atomic commits, historical loads and durable operation receipts.
- [x] Restart, concurrent stale-write, interrupted transaction, late write failure and SQLite-full tests.
- [x] Configure Linux/Windows CI (remote results must be checked separately).

## Local foundation

- [ ] Select Rust server and cross-platform packaging dependencies.
- [ ] Serve the TypeScript interface on loopback only.
- [ ] Preserve three-pane layout, resize controls and dark mode.
- [ ] Preserve default Study and optional Critical review; no Exam practice mode or Study badge.
- [x] Draft logical data model and document revision lifecycle (design only).
- [x] Specify read-only document, typed schema and operation contract (design only).
- [ ] Complete the full closed schema and common graph/chat validator (six-operation experimental subset implemented).
- [ ] Select/document portable Markdown syntax and test round-trip graph reconstruction.
- [ ] Implement read-only document pane; remove direct source-edit mutation routes.
- [ ] Define bounded history retention, dependency protection and storage accounting.
- [ ] Review the data/revision proposals and translate them into SQL constraints and migrations.
- [ ] Define workspace schema, migrations and relative-path rules.
- [ ] Complete Markdown publication and workspace revision lifecycle (SQLite document/claim persistence implemented).
- [ ] Implement idempotent expected-head saves, durable publication jobs and external-edit reconciliation.
- [ ] Validate anchor mapping and immutable citation targets against the revision acceptance cases.
- [ ] Import prototype workspace JSON without losing content.
- [ ] Implement consistent backup/restore and test Linux-to-Windows migration.

## AI and credentials

- [ ] Move DeepSeek requests into a Rust provider adapter.
- [ ] Recheck current deepseek-flash availability and record returned model metadata.
- [ ] Integrate OS credential stores, including unavailable/locked Linux service handling.
- [ ] Add explicit session-only fallback, key replacement and deletion.
- [ ] Preserve usage reporting, cancellation, validated output and actionable errors.
- [ ] Preserve concurrent chat/graph changes with version checks.
- [ ] Test real learning sessions with credentials supplied through the app.

## Security audit — not performed

- [ ] Audit key handling in memory, logs, errors, exports and crash reports.
- [ ] Audit loopback binding, Origin/Host validation, sessions, CSRF and DNS rebinding.
- [ ] Audit filesystem/import access, path traversal and resource limits.
- [ ] Audit Markdown/math rendering, XSS and untrusted source instructions.
- [ ] Audit provider destinations, cancellation and rate limits.
- [ ] Review dependencies and release/update provenance.
- [ ] Document findings, fix issues and independently verify remediation before claiming security auditing is complete.

## Retrieval

- [ ] Import Markdown, text and text-based PDFs with source metadata.
- [ ] Add passage/context viewing and resolvable citations.
- [ ] Implement lexical search and per-document source selection.
- [ ] Establish a representative retrieval evaluation set.
- [ ] Select local embeddings based on hardware and language tests.
- [ ] Add hybrid ranking, deduplication and bounded expansion.
- [ ] Add graph-assisted retrieval without making graph extraction mandatory.
- [ ] Preserve lineage and prevent summary self-corroboration.

## Knowledge and reasoning

- [ ] Stable anchors and version-specific assessments.
- [ ] Separate provenance, curation, assessment and hypothetical acceptance.
- [ ] Reviewable AI relationships and explicit multi-premise inference.
- [ ] Corrections, counterexamples and proof-gap distinctions.
- [ ] Versioned scenarios with traces, pinned inputs, explicit refresh and conflict containment.
- [ ] Conditional-result preservation, cycle handling and explicit rules for negation/contraposition.
- [ ] Scenario comparison, disagreement tracing and dependency impact review.
- [ ] Hide assessments during Study without changing stored values.

## Validation

- [ ] Browser checks for resizing, editing, citations, themes and keyboard access.
- [ ] Persistence, disk-full, interrupted-write and recovery tests.
- [ ] Mathematical, interpretive and language-learning workflows.
- [ ] English presentation with multilingual/right-to-left source material.
- [ ] Accessible textual equivalents for graph colors.
- [ ] Distinguish unit checks, browser testing, live-provider testing and security audit.

## Later

- [ ] Optional MCP/API access for Codex or Claude Code.
- [ ] OCR and advanced equation/PDF extraction.
- [ ] Formal proof-assistant integration.
- [ ] Richer learning progress, open questions, checkpoints and review scheduling.
- [ ] Definition/terminology views and a focused claim/counterexample workspace.
- [ ] Publication-oriented LaTeX/PDF export.

Cloud synchronization is not required. Portable local backups are required.
