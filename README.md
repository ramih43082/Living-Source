# Living Source

A local, AI-assisted learning workspace for any subject. Conversation drives the work; a read-only learning document updated through chat and graph actions and an inspectable knowledge graph develop alongside it.

**Status:** first Rust document-core implementation with a runnable command-line demo and generated schemas. SQLite persistence and historical revision loading are implemented. The local browser application and deep RAG are not implemented yet. The hosted prototype remains a separate design reference.

## Run the first core

Install Rust through rustup; the pinned toolchain is specified in rust-toolchain.toml.

```sh
cargo test --workspace --locked
cargo run --locked -p living-source-core --example demo
cargo run --locked -p living-source-core --example persistent_demo -- workspace.sqlite
```

See [implementation scope and developer setup](docs/IMPLEMENTATION.md). The original demo is in-memory. persistent_demo saves to SQLite and reloads the same document on subsequent runs. Neither needs an API key. [Generated schemas](schemas/proposal.schema.json) describe the supported experimental subset; the broader specifications below remain the target.

## Planning documents

- [Contributor guide](CONTRIBUTING.md)
- [Requirements and acceptance examples](docs/REQUIREMENTS.md)
- [Product plan](PLAN.md)
- [Local architecture](docs/ARCHITECTURE.md)
- [Data model draft](docs/DATA_MODEL.md)
- [Document revision lifecycle](docs/DOCUMENT_REVISIONS.md)
- [Structured document schema](docs/DOCUMENT_SCHEMA.md)
- [Allowed editing operations](docs/EDIT_OPERATIONS.md)
- [RAG implementation](docs/RAG.md)
- [Roadmap and TODO](TODO.md)
- [Decisions and open questions](docs/DECISIONS.md)

## Fixed requirements

- Local website for one user, opened in a browser on Linux or Windows 11.
- Rust backend and a reusable TypeScript web interface.
- Chat fills the left half; document above graph on the right half.
- English-only presentation; other languages are supported as source and study material.
- Markdown with LaTeX math. The AI proposes typed changes; the learner directs changes through chat and graph controls. The document pane is read-only.
- Study by default, with an optional Critical review toggle.
- No separate Exam practice mode and no redundant Study mode badge.
- Manual promotion of useful results into the curated source.
- Local storage and portable backups.
- DeepSeek as the initial conversational provider.

The [hosted prototype](https://rami-living-source.leapinggerbil.chatgpt.site) demonstrates the interaction design. Its hosted backend and browser-local storage are temporary, not the target architecture.

Do not commit API keys, credentials, personal learning data, imported books, or workspace databases to this repository. Security auditing is an **open task**, not an accomplished certification.

Plan consolidated from the design discussion of 13 November 2026; moved to GitHub on 14 November 2026.
