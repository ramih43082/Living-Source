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
4. Validate the proposed revision/anchor reconciliation design on real Markdown and both OSs.
5. Validate managed import copies for v1; externally linked references are deferred in the data-model proposal.
6. The initial restricted inference language.
7. Workspace retention and backup defaults.

These decisions can be resolved when they affect concrete implementation. No cloud synchronization requirement or full code rewrite of the interface is implied.

## Completeness and contributor handoff

The [requirements checklist](REQUIREMENTS.md) maps the available discussion to acceptance examples. Optional ideas are listed separately from accepted behavior. [CONTRIBUTING](../CONTRIBUTING.md) explains the planning-only state, implementation starting point and evidence required to mark work complete.

## Proposed data and revision decisions — 14 November 2026

These are implementation proposals, not user-ratified changes or completed code:

- Separate persistent entity IDs from immutable version IDs; citations and assessments pin versions.
- Keep complete committed Markdown snapshots in SQLite, exporting portable working copies through durable jobs.
- Use expected-head checks and idempotent saves; hold stale AI edits for review.
- Preserve block lineage without treating structural continuity as semantic equivalence.
- Copy reference originals into managed storage initially; keep external Markdown reconciliation conservative.
- Keep curation, assessment, hypothetical admission and source lineage separate.
- Archive rather than cascade-delete cited history; defer permanent erasure and retention policy.

See [data model](DATA_MODEL.md) and [revision lifecycle](DOCUMENT_REVISIONS.md). Exact SQL, migrations, filesystem durability, resource limits and the inference language remain to be specified and tested.
