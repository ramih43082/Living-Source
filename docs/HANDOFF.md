# Contributor handoff

Prepared for Rami's programmer friend and other contributors on 14 November 2026. The task at this point is to understand and review the feature specification; implementation can resume later.

## Read in this order

1. [FEATURES](FEATURES.md): full user-facing intent, examples and boundaries.
2. [DECISIONS](DECISIONS.md): latest accepted choices and unanswered decisions.
3. [FEATURE_STATUS](FEATURE_STATUS.md): code/prototype/planned distinctions and evidence.
4. [DOCUMENT_SCHEMA](DOCUMENT_SCHEMA.md): four viewer types and JSON direction.
5. [IMPLEMENTATION](IMPLEMENTATION.md) and [STORAGE](STORAGE.md): actual existing code, commands and limits.
6. [RAG](RAG.md): later retrieval design, not a first-viewer dependency.

## What changed most recently

- First usable milestone is a document viewer, not the complete AI workspace.
- JSON is the document format. Markdown/LaTeX are allowed inside content fields.
- MyST, custom Markdown syntax and Logseq compatibility are not current work items.
- The document is read-only; later changes occur through chat/graph operations.
- Initial viewer types: Section, Explanation, Claim and Reasoning.
- Owner now wants a detailed handoff rather than further implementation.

These supersede conflicting older recommendations, examples and prototype behavior.

## What is already here

A Rust workspace with a six-operation document engine and SQLite persistence. It creates documents, sections and claims, versions claims, selects displayed claim versions, validates proposals, rejects stale updates and handles transaction/retry failures. There are CLI demos, generated JSON Schemas, tests and CI configuration.

It is not a browser application, a complete graph, a retrieval pipeline, an AI authoring service or a security-audited release. No coding should assume those layers already exist.

The existing [hosted prototype](https://rami-living-source.leapinggerbil.chatgpt.site) is visual/historical context only. Its direct document editing is superseded. Its code and personal browser data have not been imported. Obtain source/access explicitly if needed; do not assume this repository contains it.

## Proposed continuation sequence, when coding resumes

1. Review D01/D07/D15–D17 with the owner and agree concrete viewer acceptance examples.
2. Resolve viewer loading workflow and layout, then define a versioned four-type JSON contract.
3. Create non-private fixtures covering mathematical, interpretive and multilingual documents, including malformed inputs.
4. Build safe read-only rendering and reference inspection. Use fixtures without waiting for AI or RAG.
5. Connect storage if required by the chosen loading workflow.
6. Agree the next milestone with the owner before expanding graph, chat, curation or retrieval.

This is a recommended sequence, not a blanket assignment to build all later layers.

## How to use the specification

Use F IDs in issues/PRs and D IDs in decisions. State whether work implements accepted behavior, explores a proposal or addresses an unresolved choice. Attach acceptance evidence and update FEATURE_STATUS when merging.

Do not equate source inclusion, verification, model confidence and hypothetical acceptance. Do not conflate a displayed reasoning block with an evaluated proof. Do not make the system math-only because the easiest fixtures are mathematical.

Keep code-derived contracts explicit: generated schemas describe implemented types, while the product spec can describe future types. An adapter/version change is required when these differ.

Use existing tests as regression checks, but add tests appropriate to actual changes. Browser rendering, credential handling, provider behavior, backup recovery and cross-OS delivery require separate evidence.

## Ownership and open questions

The repository URL was supplied by Rami and is private. This handoff does not invite users, change permissions, publish the project or grant a license. Rami manages collaborator access. An open-source implementation was requested, but no specific license was approved.

The decision queue is in DECISIONS. The developer should make routine implementation choices within agreed constraints, while escalating concrete choices about behavior, scope, privacy, portability or destructive retention.

## Documentation precedence

1. Latest explicit owner decisions recorded in DECISIONS.
2. FEATURES and the current feature register.
3. Focused current design notes.
4. Existing implementation contracts for describing what code currently accepts.
5. Archived drafts for historical rationale only.

If code and product intent differ, report the gap; do not silently redefine the product to match the code. Old drafts are preserved under archive and must not be used as active requirements.
