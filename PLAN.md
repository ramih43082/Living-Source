# Product plan

The detailed current plan is [docs/FEATURES.md](docs/FEATURES.md). This entry point summarizes its priorities.

## First usable version

A read-only JSON document viewer showing Section, Explanation, Claim and Reasoning content, with Markdown prose and LaTeX equations inside content fields. Citations attach to content. The initial loading workflow and viewer layout still need agreement.

AI authoring, interactive graph editing, RAG and hypothetical evaluation are later milestones, not prerequisites for this viewer.

## Full product direction

A local single-user learning workspace for arbitrary subjects on Linux and Windows 11. Chat occupies the left half; document and graph share the right vertically. Changes originate through chat/graph operations; the document remains read-only.

English presentation, multilingual source/study material, DeepSeek initially, manual source promotion, contextual claim-level assessments, versioned dependencies, hypothetical scenarios, local retrieval and portable backups remain in scope.

## Latest changes

JSON replaces Markdown/MyST/Logseq as the document format. Markdown/LaTeX remain allowed inside content. Do not implement a custom Markdown parser or MyST export as an assumed next task.

The owner has requested documentation for incoming contributors; implementation may resume later.

Read [HANDOFF](docs/HANDOFF.md), [DECISIONS](docs/DECISIONS.md), [FEATURE_STATUS](docs/FEATURE_STATUS.md) and [TODO](TODO.md). Existing code is a partial Rust/SQLite core, not a complete local application.
