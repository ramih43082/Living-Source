# Product plan

The local architecture in this plan supersedes the earlier hosted, browser-storage-first implementation direction.

## Purpose and scope

Living Source is a general-purpose learning and knowledge-building workspace. Mathematics, physics, philosophy, biblical studies, literature, and language learning illustrate the range; they are not exclusive categories or mandatory document templates.

A learning document grows primarily through discussion between the learner and AI. The AI writes and revises it; the learner can edit directly. Conversation, working documents, reference materials, and curated claims remain distinct but connected.

## Workspace

- Left half, full height: chat, the primary interaction.
- Upper-right: current learning document.
- Lower-right: relevant knowledge graph.
- Draggable dividers, temporary pane maximization, accessible narrow-screen navigation.
- Light/dark appearance with a remembered preference.
- Selecting a passage supplies conversational context and should highlight related graph nodes.
- Selecting a graph node should reveal its passage, provenance and reasoning.
- Show the relevant graph neighborhood rather than the entire collection by default.

English is the sole presentation language for UI, explanations, headings, instructions and feedback. Original quotations, vocabulary, dialogues and exercises may use other languages with English framing.

## Learning documents

Create independent documents for any topic or goal, starting blank, from notes or references, or from an AI-proposed outline. Specialized structures are optional.

Support explanations, equations, definitions, proofs, arguments, interpretations, examples, exercises, vocabulary, open questions, and references. Documents share a knowledge base and may link to existing claims without duplicating them as independent evidence.

Use Markdown plus LaTeX math, with rendered and source views. Full LaTeX export is a possible later feature.

Required editing behavior:
- AI can update the working draft when enabled.
- Learner edits are preserved.
- AI changes prepared from an older version are held for review rather than overwriting intervening edits.
- Revision history, undo and comparison.
- Stable block/passage identities and recoverable stale anchors.
- Portable import/export and backup restoration.

Writing a document is not the same as promoting a result into the curated source. Working text can develop naturally; the learner explicitly controls source inclusion and verification.

## Study and Critical review

Study is the default behavior. Use selected documents as the material being studied: explain, summarize, give examples and support understanding. No assumption-setting or verification step is required.

Critical review is an optional toggle. It reveals assessments, challenges, evidence, dependencies, and hypothetical reasoning. Turning it off hides this workflow; it does not erase assessments or mark material verified.

There is no separate Exam practice mode. The learner can request exercises through discussion without a dedicated exam interface or special restrictions on document writing. There is no redundant Study mode badge.

Studying a source as provided does not establish its factual truth. Answer explicit correction requests honestly; identify contradictions that prevent a coherent explanation without turning routine study into an audit.

## Sources, claims and trust

Keep provenance, learning value, curation and assessment separate. A source can contain factual errors and remain valuable. Assess particular claims and passages rather than assigning a single truth value to a book.

New claims and relationships begin unverified in the assessment model. Unverified means not yet assessed, not false. Hide these labels during ordinary study.

Assessment records identify:
- Exact claim/version and scope.
- Judgment and method.
- Reviewer, date and rationale.
- Supporting evidence.
- Challenges and superseding assessments.

Possible judgments: unverified, supported, verified within stated scope, disputed, refuted within stated scope. “Verified by me” identifies the reviewer; formal checking is a separate method.

Standards depend on the domain: proof review for mathematics, evidence and methodology for empirical claims, textual support for interpretation, and original-language comparison for translation.

A manual Add to source action lets the learner review the statement, assumptions, proof or explanation, evidence and destination. Inclusion does not itself verify the entry. AI agreement, repetition and source prestige do not automatically upgrade status.

## Knowledge graph

Connect specific claims and their versions, with links back to sources. Distinguish:
- Explicit conditional inference.
- Derivation dependency.
- Evidential support.
- Challenge or refutation.
- Interpretation.
- Citation.
- Equivalence under stated scope.

A AND B implies C must be one inference object requiring both premises, not two arrows each independently establishing C.

Preserve differences between an author's assertion, a reported speaker's statement, a metaphor, and our reconstruction. Edition, translation, speaker, time, genre and interpretive scope can affect relationships.

Graph construction is gradual. A document can be imported and studied before its claims have been extracted or mapped.

## Hypothetical reasoning

Scenarios are versioned overlays, separate from permanent assessments. The learner temporarily accepts premises and explicitly admits rules.

| Display | Meaning in the scenario |
| --- | --- |
| Blue | Explicit starting assumption |
| Green | Positive conclusion follows through eligible rules |
| Red | Its explicit negation follows |
| Gray | Neither direction established in the evaluated scope |
| Amber | Conflict, or a clearly identified provisional interpretation |

Use labels/icons as well as color. Preserve an assumed badge when a conflict exists.

Compatibility is not entailment. Citation, similarity, evidential support and interpretation alone cannot propagate logical truth. Proposed AI connections remain provisional until reviewed or deliberately admitted for conditional exploration.

For “accept this book,” create a reviewable, versioned claim set and interpretation first. Show extraction coverage. Do not imply the system has captured every possible meaning of every passage.

Each result exposes its premises, rule, conditions, source versions and reasoning. Support scenario comparison and tracing opposing conclusions to differing premises or interpretations.

Scenarios pin claim and rule versions. Updating the source does not silently rewrite an existing scenario; offer an explicit refresh with an impact review. A saved conditional result retains its premises, rules and interpretive scope even when manually promoted.

Cycles do not establish their own premises. Do not infer converses or contrapositives unless explicitly licensed by the selected reasoning rules. A book-level summary must show mixed outcomes and coverage rather than a single green/red truth value.

Track positive and negative derivations independently. Contain contradictions rather than allowing unrestricted classical explosion into unrelated claims. Bound evaluation and distinguish incomplete runs from genuinely unresolved results.

## Corrections and challenges

Link challenges to exact claims and versions; assess challenges themselves.

- A valid counterexample must satisfy all assumptions while violating the conclusion.
- A proof gap makes that derivation incomplete; it does not alone prove the theorem false.
- Contrary evidence must address the actual claim and scope.
- Preserve originals and retrieve stored corrections with their targets.
- Flag dependent derivations for review without automatically declaring their conclusions false.
- Keep old verification on its old version; changes trigger reassessment.

A conditional relationship can be verified without accepting the truth of its premises.

## AI

DeepSeek is the initial provider. The prototype uses `deepseek-flash`, documented during setup as the rolling alias for V4.1 Flash 1017. Recheck availability when connecting and record returned model metadata; do not claim the alias permanently pins a snapshot.

Provider integration should be replaceable. Do not silently substitute models; report the active model and any user-approved fallback. The application owns documents, conversations, graph, assessments and history.

Require English presentation, traceable context, actionable errors, usage reporting, cancellation, validated proposals, conflict-safe revisions, and no fabricated citations. Distinguish quotation, paraphrase, synthesis and hypothetical inference.

Local storage does not mean inference is offline: the API credential and selected conversation/source context are sent to DeepSeek.

## Current implementation boundary

The hosted prototype demonstrates panes, document creation/editing, DeepSeek connection, revision protection, manual curation, basic scenarios, Study/Critical review, exports and themes.

The local Rust runtime, portable persistent workspace, OS credential storage, deep RAG, richer cross-document graph and Linux/Windows packaging remain to be built. The prototype is not security-audited, and live-provider and browser validation remain incomplete.

Prioritize dependable local persistence, the chat-to-document workflow, resolvable citations and evaluated retrieval. Defer automatic formalization of whole books, unrestricted theorem proving, advanced OCR and exhaustive scholarly adjudication.
