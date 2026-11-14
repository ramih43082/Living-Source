# Living Source

A local learning workspace where discussion develops a structured document and inspectable knowledge graph across arbitrary subjects.

**Current priority:** detailed handoff to incoming contributors. The first usable milestone is a **read-only JSON document viewer**. Implementation can continue later.

## Start here

1. [Contributor handoff](docs/HANDOFF.md)
2. [Detailed feature specification](docs/FEATURES.md)
3. [Feature status and code evidence](docs/FEATURE_STATUS.md)
4. [Decisions and open questions](docs/DECISIONS.md)
5. [Work register](TODO.md)

## What exists

A Rust library/CLI with six validated document/claim operations and SQLite persistence. It supports versioning, atomic changes, historical reads and durable retries. Recorded local validation: 19 tests, formatting and Clippy checks passed.

There is no local browser viewer, complete graph, AI adapter or RAG pipeline yet. The [earlier hosted prototype](https://rami-living-source.leapinggerbil.chatgpt.site) is a separate design reference; its source has not been migrated and its direct document editing is superseded.

## Current product decisions

- JSON document structure; Markdown/LaTeX inside content fields.
- First viewer types: Section, Explanation, Claim and Reasoning.
- Read-only document; eventual edits through validated chat/graph operations.
- English presentation with multilingual study/source material.
- Local Linux/Windows 11 browser application; Rust backend.
- Eventual chat-left, document-above-graph-right workspace.
- Study default, optional Critical review; no dedicated Exam practice.
- Manual curation, contextual claim-level assessment and hypothetical reasoning.
- DeepSeek initially; local persistence/retrieval and external generation.

MyST, custom Markdown file syntax and Logseq compatibility are not current goals. No complete security audit has been performed.

## Existing core commands

Install Rust through rustup and a C compiler for bundled SQLite. From the repository root:

```sh
cargo test --workspace --locked
cargo run --locked -p living-source-core --example demo
cargo run --locked -p living-source-core --example persistent_demo -- workspace.sqlite
```

See [implementation setup](docs/IMPLEMENTATION.md) and [storage boundaries](docs/STORAGE.md). The persistent demo reloads data; it is not the viewer.

Do not commit credentials, private conversations, imported books or workspace databases. Collaborator permissions and the project license remain owner decisions.

Further notes: [architecture](docs/ARCHITECTURE.md), [data model](docs/DATA_MODEL.md), [JSON schema direction](docs/DOCUMENT_SCHEMA.md), [operations](docs/EDIT_OPERATIONS.md), [revisions](docs/DOCUMENT_REVISIONS.md), [RAG](docs/RAG.md), [requirements mapping](docs/REQUIREMENTS.md), [contributing](CONTRIBUTING.md).
