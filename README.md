# Living Source

A local, AI-assisted learning workspace for any subject. Conversation drives the work; an editable learning document and an inspectable knowledge graph develop alongside it.

**Status:** planning repository. The hosted prototype is a design reference. The local Rust application and deep RAG system have not yet been implemented here.

## Planning documents

- [Contributor guide](CONTRIBUTING.md)
- [Requirements and acceptance examples](docs/REQUIREMENTS.md)
- [Product plan](PLAN.md)
- [Local architecture](docs/ARCHITECTURE.md)
- [RAG implementation](docs/RAG.md)
- [Roadmap and TODO](TODO.md)
- [Decisions and open questions](docs/DECISIONS.md)

## Fixed requirements

- Local website for one user, opened in a browser on Linux or Windows 11.
- Rust backend and a reusable TypeScript web interface.
- Chat fills the left half; document above graph on the right half.
- English-only presentation; other languages are supported as source and study material.
- Markdown with LaTeX math. The AI is the main writer; the learner can edit.
- Study by default, with an optional Critical review toggle.
- No separate Exam practice mode and no redundant Study mode badge.
- Manual promotion of useful results into the curated source.
- Local storage and portable backups.
- DeepSeek as the initial conversational provider.

The [hosted prototype](https://rami-living-source.leapinggerbil.chatgpt.site) demonstrates the interaction design. Its hosted backend and browser-local storage are temporary, not the target architecture.

Do not commit API keys, credentials, personal learning data, imported books, or workspace databases to this repository. Security auditing is an **open task**, not an accomplished certification.

Plan consolidated from the design discussion of 13 November 2026; moved to GitHub on 14 November 2026.
