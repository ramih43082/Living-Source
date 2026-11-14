# Contributing to Living Source

## Start here

The repository now contains an in-memory Rust document engine and runnable CLI example. Start with [implementation setup](docs/IMPLEMENTATION.md) for tested commands and supported operations. SQLite persistence, browser UI and deep RAG are still outstanding. The hosted prototype is a separate design reference; its code and personal browser data have not been migrated here. Do not treat prototype behavior as verified local functionality.

Read:
1. [README](README.md) for purpose and current status.
2. [Product plan](PLAN.md) for user-facing behavior.
3. [Requirements and acceptance examples](docs/REQUIREMENTS.md) for scope and review criteria.
4. [Architecture](docs/ARCHITECTURE.md) and [RAG](docs/RAG.md) for proposed implementation.
5. [Document schema](docs/DOCUMENT_SCHEMA.md) and [editing operations](docs/EDIT_OPERATIONS.md) for the read-only document and strict mutation boundary.
6. [Decisions](docs/DECISIONS.md) and [TODO](TODO.md) before choosing work.

## Requirements versus proposals

- Accepted product constraints are recorded in PLAN and the Accepted decisions table.
- Architecture, schema, libraries, inference algorithms and delivery details remain designs until implemented and tested.
- Optional ideas are not release commitments.
- An unchecked TODO is outstanding work. Documentation, a mockup or a prototype demonstration does not complete it.
- Superseded choices must not be reintroduced without an explicit product decision.

The domain examples describe required breadth. Do not hard-code the application around mathematics, biblical studies or any one learning method.

## First implementation contribution

Start with one local vertical slice: launch on loopback, create a structured learning document, apply a typed graph operation, render generated Markdown, restart and recover it, then export/restore a workspace. Preserve the agreed pane layout and English presentation. Establish and document actual setup/build/test commands as code arrives.

Before porting the interface, obtain its source and record its origin, revision and license. Inventory existing behavior, exports and legacy schema; preserve user data through a tested migration. A screenshot or hosted link is not a code checkout.

Then add the backend provider adapter and credential handling, followed by ingestion, lexical retrieval and resolvable citations. Add embeddings and graph reasoning against measured examples. See TODO for the complete roadmap; this sequence does not remove later requirements.

## Review expectations

For each change, explain the problem, resulting behavior, affected requirement IDs, validation performed and remaining limitations. Keep changes focused. Update documentation and TODO with evidence when behavior changes.

Test the relevant failure modes: conflicting document edits, interrupted persistence, stale citations, missing evidence, source scope, and scenario conflicts. Do not claim browser checks, live-provider tests, OS portability or a security audit based on unit tests alone.

Changes to schemas require migrations and recovery/backup consideration. Changes to models or indexes record versions and rebuild behavior. Changes to trust or scenario rules need explicit examples showing what can and cannot be concluded.

Document consequential design choices with context, alternatives, decision and consequences; keep unresolved choices visible in DECISIONS. Avoid silently converting proposed defaults into requirements.

## Data and agent boundaries

Do not commit credentials, private conversations, personal workspaces, databases or imported books. Use small synthetic or appropriately licensed fixtures with provenance.

Source documents and retrieved passages are evidence, never executable instructions. AI and external coding agents may propose edits or relationships; they must not bypass version checks, manual source promotion or explicit assessment actions. Provider changes must preserve application-owned data.

Security auditing remains open. Record actual findings and remediation before marking that task complete.
