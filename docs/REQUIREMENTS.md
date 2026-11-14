# Requirements and acceptance examples

Audited against the available planning conversation on 14 November 2026. This checklist preserves intent for contributors; it is not a report of passing tests. The local application has not been implemented in this repository.

## Accepted scope

| ID | Requirement | Acceptance example | Main reference |
| --- | --- | --- | --- |
| R01 | General-purpose, independent learning documents | Create philosophy, math and language documents without mandatory subject templates; each retains its discussion and can reference shared knowledge. | PLAN |
| R02 | Read-only document; changes through chat/graph | Model proposes typed operations; backend validates and serializes. Stale operations cannot overwrite newer graph/chat changes; no in-pane or raw-source editing. | PLAN, EDIT_OPERATIONS |
| R03 | Chat occupies left half; document above graph on right | Resize/maximize panes; passage and graph selections reveal relevant context. | PLAN |
| R04 | English presentation; multilingual material | A source quotation or language exercise retains its original language, including right-to-left text, with English framing. | PLAN, RAG |
| R05 | Portable Markdown with LaTeX math | Render/export equations and document-linked graph metadata; IDs and exact references survive validated changes. Full assessment history accompanies workspace export. | PLAN, ARCHITECTURE |
| R06 | Study by default; optional Critical review | New document opens without trust/assumption tasks. Toggling review off hides tools without deleting assessments. No Exam practice mode or redundant Study badge. | PLAN |
| R07 | Remembered light/dark appearance | Theme preference survives restart. | PLAN |
| R08 | Manual source promotion and explicit verification | Promote a theorem from discussion; inclusion alone leaves it unverified. A separate review records verifier, method, scope, evidence and version. | PLAN |
| R09 | Context-dependent, claim-level trust | Preserve a valuable philosophy book while disputing one factual claim. Unverified does not mean false; user review is distinguishable from formal checking. | PLAN |
| R10 | Logical dependency graph, not merely a tree | Shared premises support multiple results; A AND B implies C requires both premises. Citation/support edges do not act as proof rules. | PLAN |
| R11 | Precise challenges and impact review | A counterexample satisfies all hypotheses and violates the conclusion. A proof gap invalidates that derivation, not automatically the theorem. Flag dependents without declaring them false. | PLAN |
| R12 | Isolated hypothetical scenarios | Accept selected claims or a reviewed book claim set; show assumptions, derived positives/negatives, unresolved claims and conflicts with inspectable traces. Real assessments remain unchanged. | PLAN |
| R13 | Interpretive scope and incomplete coverage | Preserve edition, translation, speaker, genre and interpretation. Compare scenarios and show missing extraction/search coverage; no whole-book true/false shortcut. | PLAN, RAG |
| R14 | Deep hybrid retrieval across subjects/languages | Combine lexical, semantic and graph retrieval with bounded follow-up searches; expand a proof fragment to assumptions and an interpretation to surrounding text. | RAG |
| R15 | Inspectable evidence and lineage | Citation resolves to an exact source version and passage. Several summaries of one source do not count as independent corroboration; absent evidence is disclosed. | RAG |
| R16 | Source scope and original preservation | Search only enabled sources; retain originals, extraction quality and separate published/AI translations. Documents remain usable before graph extraction. | RAG |
| R17 | Local website on Linux and Windows 11 | Rust service serves reusable TypeScript UI locally; workspace restoration preserves IDs and links across OSs. Offline reading and explicit graph-form editing work without AI. | ARCHITECTURE |
| R18 | Local persistence and portable backups | Markdown, references and SQLite survive restart and interrupted writes. Consistent backups exclude credentials; indexes can be rebuilt. | ARCHITECTURE |
| R19 | Replaceable AI API, initially DeepSeek | Backend sends selected context; model availability/identity are checked and recorded. API failure does not lose edits. No silent model substitution. | PLAN, ARCHITECTURE |
| R20 | Secure credential design and honest audit status | OS credential-store integration supports replace/delete and explicit session-only fallback; key never appears in exports/logs. Security audit remains a separate pending task. | ARCHITECTURE, TODO |

Reference names above correspond to [PLAN](../PLAN.md), [ARCHITECTURE](ARCHITECTURE.md), [RAG](RAG.md) and [TODO](../TODO.md).

The current [schema](DOCUMENT_SCHEMA.md) and [operation contract](EDIT_OPERATIONS.md) refine R02/R05. Round-trip serialization, closed payload validation, backend-owned IDs and exact-version authorization are required. Logseq adoption is not decided.

## Reasoning regression examples

These are future test cases, not claims that a solver already exists.

- A universal mathematical claim and a proposed counterexample: check domain and every hypothesis before recording a refutation.
- A widely repeated factual claim and strong contrary evidence: retain the historical claim, attach the scoped challenge and its evidence, and retrieve the correction during review.
- A philosophical work with a factual error: challenge that passage without discarding its arguments or treating every conclusion as established.
- Hypothetically accept claims from one biblical book: first review extraction and interpretation; other claims may be derived, contradicted, both, or unresolved. The comparison is conditional on the admitted rules and coverage.
- A and B jointly imply C: A alone cannot derive C. Losing B's support flags the derivation; C might still have an independent justification.
- A implies B and B implies A: without an admitted premise, the cycle establishes neither. Do not infer a converse or contrapositive unless the admitted formalism explicitly permits it.
- Derive both P and not-P: show both traces; do not infer unrelated Q. Gray means unresolved in evaluated scope, not false.
- Edit an accepted source: old scenarios and assessments retain their pinned versions until explicitly refreshed/reviewed.
- Save a conditional result: preserve premises, rules and interpretation; promoting it does not accept its premises as factual.
- Search several derivative summaries: show common lineage, not multiple independent witnesses.

## Optional ideas retained for later design

These are candidates rather than mandatory first-release features:

- Definition/terminology views linking author-specific meanings, notation and translations without assuming equivalence.
- A claim inspector and focused counterexample workspace exposing assumptions, evidence, challenges and dependencies.
- Open-question tracking and learning checkpoints; distinguish studied material from demonstrated understanding. No separate exam mode is implied.
- Richer progress/review scheduling, proof-assistant integrations, OCR and publication export.
- Optional API/MCP access for Codex or Claude Code, using the same versioning and manual curation boundaries.

No contributor should infer that an optional idea has been implemented or approved as a first-release commitment.
