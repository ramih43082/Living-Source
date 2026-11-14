# Decision register

Updated 14 November 2026 from the available project conversation. Product owner: Rami. This register records decisions; recommendations are not silently treated as approval.

## Accepted current direction

| ID | Decision | Origin / effect |
| --- | --- | --- |
| D01 | First usable version is the structured document viewer. | Owner explicitly narrowed the earlier chat-first milestone. AI and interactive graph are not release prerequisites. |
| D07 | JSON is the document format. | Owner: “let's just stick with JSON.” Supersedes MyST, Logseq-compatible and custom Markdown file formats. |
| D15 | Document pane is read-only; changes originate through chat/graph. | Owner explicitly removed in-document editing. |
| D16 | LLM output must obey strict operations; backend validates and generates state. | Owner requested program-enforced structure, not unconstrained file generation. |
| D17 | First viewer recognizes Section, Explanation, Claim and Reasoning. | Agreed viewer direction; Markdown/LaTeX may live inside content fields. Citations attach to blocks. Exact schema is not finalized. |
| D18 | Presentation is exclusively English. | Other languages remain valid source/study content. |
| D19 | Long-term layout: chat left half, document above graph on right. | Owner specified proportions and priority; viewer-only layout remains open. |
| D20 | Local single-user browser application targeting Linux and Windows 11. | Owner wants local data and OS portability. Rust backend is the chosen direction; TypeScript interface reuse is technical guidance. |
| D21 | DeepSeek is the initial conversational provider. | Verify current model identity when implementing; do not hard-code a historical alias as a guaranteed snapshot. |
| D22 | Explicit manual promotion into curated source. | Inclusion is independent of verification. |
| D23 | Context-dependent claim-level trust; unverified until assessed. | Valuable sources with errors are retained. No blanket truth judgment for a book. |
| D24 | Study default, optional Critical review. | Hide verification/assumption workflow in ordinary study; do not change stored trust. |
| D25 | Hypothetical acceptance and dependency exploration are part of the full product. | Mathematical and interpretive use cases; not part of first-viewer evaluation. |
| D26 | Dark mode option. | Owner requested it in the prototype; retain as product requirement. |
| D27 | Keep a security audit on the roadmap. | Explicit owner request; no audit has been completed. |
| D28 | Focus now on detailed documentation for incoming programmers. | Latest owner instruction; implementation can come later. |

## Previously numbered decisions still open

The earlier D01–D14 questionnaire contained recommendations. Only decisions explicitly settled afterward are marked accepted above.

| ID | Question requiring a decision | Recommendation on record, not approval |
| --- | --- | --- |
| D02 | Which AI changes may auto-apply, and which need review? | Auto-apply fresh narrative when enabled; review changes to existing claims/rules or removals. |
| D03 | What exact edits can be entered in graph forms without AI? | Allow exact statements, assumptions and relationships; graph-origin changes are already accepted in principle. |
| D04 | When does narrative become a structured claim? | AI suggests; explicit creation makes a claim object; source promotion remains separate. |
| D05 | How are curated collections organized? | Shared workspace with named collections and explicit document scope. |
| D06 | How should other documents react to a revised shared claim? | Keep existing versions pinned and offer a reviewed update. Versioning foundation exists, but cross-document UX is undecided. |
| D08 | What is in a portable document export? | Decide single JSON, JSON-plus-assets bundle, dependency inclusion and history boundaries. Previous Markdown recommendation is obsolete. |
| D09 | History retention/checkpoint defaults? | Thin unreferenced intermediates; protect evidence/scenario dependencies. No periods/counts agreed. |
| D10 | Backup destination, frequency and retention? | Automatic local backups plus explicit export/restore. No schedule agreed. |
| D11 | Which source content may leave the device for AI? | Explicit enabled scope and inspectable context; a never-send flag was suggested, not accepted. |
| D12 | Which representative subjects/languages validate the first version? | Choose actual mathematical, interpretive and language-learning examples; examples are not topic restrictions. |
| D13 | Fallback when OS credential store is unavailable? | Explicit session-only key; no silent plaintext persistence. |
| D14 | First-release essentials versus deferred features? | Reframe around accepted viewer-first D01; agree rendering/import/accessibility/packaging acceptance. Old chat-first priority recommendation is superseded. |

## Technical proposals awaiting design/validation

- Stable entity IDs separate from immutable version IDs; pin evidence and assessments.
- One SQLite workspace database; normalized typed records and portable JSON.
- Expected-version checks, atomic batches and idempotency keys.
- Multi-premise rule objects, signed derivations, conflict containment and bounded evaluation.
- Local hybrid retrieval and model/index provenance.
- OS credential adapter and explicit local-service security boundary.
- Managed reference copies rather than external links.
- Retention based on full dependency closure.

Some are partially coded. Implementation alone is not evidence that every policy/default has been approved. These proposals support the accepted behaviors in FEATURES; exact algorithms and user-facing workflows remain reviewable.

## Additional open engineering choices

- Initial viewer opens JSON files, selects stored documents, or both.
- Viewer-first layout versus future three-pane workspace.
- Exact four-type JSON schema, version negotiation, nesting and citation representation.
- Distinction between displayed Reasoning and executable inference rules.
- Frontend/rendering libraries, sanitization and safe math configuration.
- Browser support, packaging, startup, updates and accessibility acceptance.
- Hardware/languages for local embeddings and extraction.
- Initial inference formalism and interpretive rule-admission UI.
- Open-source license for the project and any reusable format/parser. Openness was requested; no particular license was approved.

Do not ask the owner to choose routine library details when contributors can evaluate them. Bring concrete tradeoffs affecting behavior, portability, scope or data loss to the owner.

## Superseded choices

| Earlier direction | Current replacement |
| --- | --- |
| Complete chat-to-document workflow as first release | Viewer-first D01 |
| Markdown/MyST/Logseq as authoritative document format | JSON D07 |
| Direct prose or raw-source document editing | Read-only D15 |
| AI writes arbitrary authoritative document file | Validated operations D16 |
| Browser localStorage / hosted runtime as final architecture | Local application and durable workspace |
| PDF reader as primary interface | AI/document/graph workspace; references inspected as evidence |
| Equal-width three columns | Chat occupies left half |
| Dedicated Exam practice and redundant Study badge | Default Study, optional Critical review |
| Configurable presentation language | English presentation |
| Whole-book green/red truth label | Claim-level conditional results with coverage |

Record future changes here with date, rationale, affected feature IDs and superseded decisions.
