# Feature and implementation register

Snapshot: 14 November 2026, code commit [7f60a79](https://github.com/ramih43082/Living-Source/commit/7f60a799015bdd3e687b6dc110ccd5fb877c6afc). Documentation-only handoff does not change that implementation.

Every F identifier maps to the corresponding heading in [FEATURES](FEATURES.md). “Implemented” applies only to the stated subset, not the full feature family. Prototype claims are historical descriptions, not fresh browser/live-provider validation.

| ID | Feature family | Status and precise boundary |
| --- | --- | --- |
| F01 | First JSON viewer | Planned. No local viewer exists. |
| F02 | Local Linux/Windows browser app | Partial foundation: Rust library/CLI. No server, packaged application or verified Windows delivery. |
| F03 | Layout/navigation/themes | Earlier hosted prototype only for panes/themes. Not migrated; linked navigation/accessibility unfinished. |
| F04 | English/multilingual/subject breadth | Partial: Unicode storage tested, subject-neutral claim content. No full viewer/learning tests. |
| F05 | JSON document types | Partial: generated snapshot/proposal schemas, root sections and claims. Explanation and Reasoning types absent. |
| F06 | Strict mutation boundary | Partial: six validated operations, atomic batches, stale checks, backend IDs and limits. No UI/service authorization or full operation set. |
| F07 | Document creation/organization | Partial: create/list/load/rename through library; append root sections. No UI, nested sections, removal/archive or prototype import. |
| F08 | Claim versions/provenance | Partial: immutable claim versions, explicit display selection and review markers. No assessment history or dependency traversal. |
| F09 | History/retention/recovery | Partial: historical reads, durable retries and failure tests. No restore/diff UI, drafts, pruning or storage accounting. |
| F10 | Manual source curation | Prototype-only interaction; no local curation storage/workflow. |
| F11 | Trust/verification | Planned. No local assessment records or verified status. |
| F12 | Corrections/counterexamples | Planned. Review markers are not a challenge/refutation engine. |
| F13 | Typed knowledge graph | Planned locally; earlier prototype graph is not a complete engine. No relationship/rule operations. |
| F14 | Hypothetical evaluation | Earlier prototype basic scenarios only. No local evaluator or formal guarantees. |
| F15 | Scenario history/comparison | Planned. No pinned scenario storage or trace comparison. |
| F16 | Study/Critical review | Prototype-only controls; not wired to new core. |
| F17 | DeepSeek authoring | Prototype-only provider connection. No Rust adapter or live-provider validation of typed operations. |
| F18 | Conversations | Planned locally. SQLite does not yet persist chat. |
| F19 | Reference ingestion | Planned. No extraction/import pipeline. |
| F20 | Hybrid/deep RAG | Planned. No search, embeddings, ranking, graph retrieval or evaluation corpus. |
| F21 | Evidence/citations/lineage | Planned. No evidence store, resolvable citation viewer or lineage enforcement. |
| F22 | Persistence/portability | Partial: one SQLite file, initial migration, atomic writes, historical loads. Backup/restore, asset management and OS migration absent. |
| F23 | Credentials/security | Planned; audit not performed. No OS key store or local web security layer. |
| F24 | Export/import | Planned for JSON. CLI JSON output is not a validated portable import/export product. No MyST/Markdown format work scheduled. |
| F25 | Quality/testing/distribution | Partial: recorded 19 local core/storage tests passing, clean fmt/Clippy; CI configured. Browser/live-AI/Windows packaging/security not confirmed. |
| F26 | Learning aids | Optional later; not implemented. |
| F27 | External API/MCP agents | Optional later; not implemented. |
| F28 | Contributor handoff | This documentation package. No transfer of ownership, invitations or access changes performed. |

## Code evidence

| Capability | Evidence |
| --- | --- |
| Six operations and typed model | [lib.rs](../crates/document-core/src/lib.rs) |
| Generated operation schema | [proposal.schema.json](../schemas/proposal.schema.json) |
| Generated experimental snapshot schema | [snapshot.schema.json](../schemas/snapshot.schema.json) |
| Atomic core/stale/retry/shape tests | [engine.rs](../crates/document-core/tests/engine.rs) |
| SQLite API and validation | [storage.rs](../crates/document-core/src/storage.rs) |
| Initial SQL schema | [001_initial.sql](../crates/document-core/migrations/001_initial.sql) |
| Reopen, history, retry, concurrency and interruption tests | [storage tests](../crates/document-core/tests/storage.rs) |
| SQLite-full test | Unit test in storage.rs; page-limit simulation, not physical disk exhaustion. |
| Persistent CLI example | [persistent_demo.rs](../crates/document-core/examples/persistent_demo.rs) |
| Linux/Windows CI configuration | [core.yml](../.github/workflows/core.yml); configuration is not a passing-run claim. |

Recorded test count: 11 core tests + 7 storage integration tests + 1 SQLite-full unit test. An ignored subprocess helper is deliberately invoked by the interruption test; it is not a skipped acceptance case. Tests were run during the implementation turn; this handoff does not claim a new test run.

## Important limits

- Current schemas are an experimental six-operation profile, not the final document-viewer contract.
- No service authorization exists. Engine consistency checks are not user consent.
- Historical loading is not restoration.
- Operation retries are idempotent; create_document is not yet an idempotent service command.
- Claim bodies avoid per-revision duplication, but manifests and accepted proposal content still consume growing space.
- No assessment engine exists, so do not claim that verification-transfer rules have been end-to-end implemented.
- JSON CLI output does not prove safe import, rendering or complete export.
- The separate hosted prototype's source has not been migrated into this repository.
