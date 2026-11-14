# Decisions and open questions

## Accepted

| Choice | Reason |
| --- | --- |
| Local browser application | Same interface on Linux and Windows 11 |
| Rust backend, TypeScript UI | Local services in Rust; reuse visual interface |
| English presentation | Other languages are source/study content |
| AI main writer; learner editor | Discussion develops an editable document |
| Markdown + LaTeX math | Readable, portable authored content |
| Structured records alongside text | Stable identities for trust and dependencies |
| Chat-left; document/graph stacked right | Chat is the main interaction |
| Study default, Critical review optional | Learn without verification bookkeeping |
| Manual source promotion | Working discussion is not automatically curated evidence |
| Claim-level contextual assessment | Errors and source value can coexist |
| Versioned claims and rules | Changes do not silently inherit verification |
| Local retrieval, external generation | Keep collections/indexes local; send relevant context to DeepSeek |
| DeepSeek initially | User-selected provider; replaceable adapter |
| OS credential storage | Persistent key handling target; audit remains pending |
| Portable local backups | OS migration without requiring cloud storage |

## Superseded or removed

- Hosted runtime as the target: now only a design reference.
- Browser localStorage as durable storage: replaced by planned local workspace persistence.
- PDF reader as the primary interface.
- Equal-width three-column layout.
- Dedicated Exam practice mode: it was an example of studying, not a separate requirement.
- Redundant Study mode badge.
- Configurable presentation language: always English.
- Whole-book green/red truth labels.
- Automatic verification from source prestige, repetition or AI agreement.

## Open at implementation time

1. Linux distribution and available CPU/RAM/GPU for local model selection.
2. Languages and formats for the first retrieval evaluation.
3. Rust server, extraction, embedding and vector-search libraries meeting portability/license requirements.
4. Reconciliation of external Markdown edits with stable passage IDs.
5. Copying imports into the workspace versus optional externally linked files.
6. The initial restricted inference language.
7. Workspace retention and backup defaults.

These decisions can be resolved when they affect concrete implementation. No cloud synchronization requirement or full code rewrite of the interface is implied.
