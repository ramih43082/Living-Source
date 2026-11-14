# Roadmap and TODO

Open work for the target local application. Prototype features are not evidence that these migration tasks are complete.

## Local foundation

- [ ] Select Rust server and cross-platform packaging dependencies.
- [ ] Serve the TypeScript interface on loopback only.
- [ ] Preserve three-pane layout, resize controls and dark mode.
- [ ] Preserve default Study and optional Critical review; no Exam practice mode or Study badge.
- [ ] Define workspace schema, migrations and relative-path rules.
- [ ] Implement crash-safe Markdown + SQLite persistence and revision history.
- [ ] Import prototype workspace JSON without losing content.
- [ ] Implement consistent backup/restore and test Linux-to-Windows migration.

## AI and credentials

- [ ] Move DeepSeek requests into a Rust provider adapter.
- [ ] Recheck current deepseek-flash availability and record returned model metadata.
- [ ] Integrate OS credential stores, including unavailable/locked Linux service handling.
- [ ] Add explicit session-only fallback, key replacement and deletion.
- [ ] Preserve usage reporting, cancellation, validated output and actionable errors.
- [ ] Preserve concurrent learner edits with version checks.
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
- [ ] Versioned scenarios with traces and conflict containment.
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
- [ ] Richer learning progress and review scheduling.
- [ ] Publication-oriented LaTeX/PDF export.

Cloud synchronization is not required. Portable local backups are required.
