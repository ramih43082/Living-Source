# Living Source — detailed product specification

Updated 14 November 2026. Owner: Rami. Audience: incoming programmers, designers and contributors.

This document describes the intended product, not a claim that its features already work. [FEATURE_STATUS](FEATURE_STATUS.md) records implementation evidence. [DECISIONS](DECISIONS.md) distinguishes accepted choices from proposals and unanswered questions. [HANDOFF](HANDOFF.md) explains where to start.

The immediate instruction is documentation and contributor handoff; do not interpret this specification as authorization to implement every feature now.

## Product purpose

Living Source helps one learner develop a durable body of knowledge through discussion with AI. The conversation is the main interaction in the eventual application; the resulting document is a structured, readable learning resource, and the graph exposes relationships between its claims.

The document is more than a chat summary. It should organize explanations, definitions, arguments, assumptions, examples and evidence as understanding develops. A working explanation is not automatically a curated source, and neither writing nor repetition makes a claim verified.

Mathematics, physics, philosophy, biblical studies, literature and language learning illustrate the range. They are not exclusive categories, fixed templates or a demand to formalize every subject mathematically. Documents can address unrelated topics and can eventually share relevant knowledge.

## Vocabulary

| Term | Meaning |
| --- | --- |
| Learning document | A structured authored resource developed through learning; stored/represented as JSON. |
| Explanation | Ordinary learning content, including prose, examples, quotations, lists and equations. |
| Claim | An explicitly represented statement with scope, assumptions and version identity. |
| Reasoning block | A displayed argument, proof or explanation connected to claim versions. Its existence does not prove validity. |
| Inference rule | An explicitly modeled logical relationship whose premises and conclusion can participate in a scenario under admitted rules. |
| Reference/source material | An imported original, such as a book, article, notes or text. It is not automatically trusted. |
| Curated source entry | A result the learner explicitly chooses to include in their knowledge collection. |
| Assessment | A contextual judgment about an exact claim/rule version, with evidence and reviewer information. |
| Scenario | Temporary, versioned acceptance of premises and rules for conditional exploration. |
| Graph | A view of claims, typed relationships, dependencies and evidence; not merely a tree of documents. |
| Revision | An identifiable state of a document or knowledge item; citations and assessments refer to exact versions. |

## F01 — First version: a structured document viewer

**Accepted milestone:** the first usable version is basically a document viewer with the specific formatting discussed. This replaces the earlier recommendation to deliver chat, AI and a graph editor as the first milestone.

The first viewer must display a valid structured document read-only. It must distinguish Section, Explanation, Claim and Reasoning content. Markdown prose and LaTeX math are content inside JSON, not the outer file format. Citations attach to content rather than requiring a fifth top-level type.

The viewer must not imply that displaying a proof means checking it, that a claim is curated because it is displayed, or that a reference establishes truth. Study-oriented reading is the default: no mandatory verification or assumption workflow.

**Proposed acceptance examples for this milestone:**
- Display a mathematical document containing a heading, explanation, claim with assumptions, reasoning and equations.
- Display an interpretive document with quotations, a claim and a competing interpretation.
- Display a language lesson with original-language material and English framing.
- Keep text selectable/copyable and references inspectable without allowing in-pane editing.
- Report a malformed or unsupported document without silently discarding parts or changing live data.
- Render safely, with no execution of instructions/code contained in document content.

Opening a local JSON file or selecting a stored SQLite document are possible loading workflows; the initial choice remains open. Full chat, interactive graph editing, AI, retrieval and scenario evaluation are not prerequisites for this milestone. The final three-pane layout is a later product requirement; whether the first viewer uses the full width is still open.

## F02 — Local operation and platform scope

The target is a single-user local website served by a local application and opened in a browser. Linux and Windows 11 are the intended platforms. A Rust backend is the chosen implementation direction; the existing TypeScript interface can be reused where appropriate. This does not require rewriting the entire interface in Rust.

Learning data should remain on the user's computer. Cloud synchronization is not required. External AI inference is a separate network dependency: selected prompts, conversation and source context must leave the machine when sent to the provider. Local storage must never be described as fully offline AI.

Offline document viewing should work. Eventually, explicit graph actions should be able to edit structured content without an AI call; the exact forms are still to be designed. OS migration should preserve workspace identities and references, with the API key entered again in the new OS credential store.

## F03 — Interface, navigation and appearance

The eventual workspace layout is:
- Chat on the left, occupying approximately half the width and the full height.
- Document in the upper-right.
- Relevant graph in the lower-right.

Preserve the user's priority: chat is the main interaction. Avoid equal three-column layouts. Resizing, temporary pane maximization and narrow-screen navigation are design requirements/proposals to validate with the owner; they must not distort the basic arrangement.

The document is read-only, including any raw/source inspection view. Selecting text should support discussion and evidence inspection. Selecting a graph object should reveal its corresponding passage, source, exact version and relationships. A focused neighborhood should be the normal graph view; showing every node at once is not useful for large workspaces.

Light/dark appearance and a remembered preference are required. Color should not be the sole carrier of graph meaning: use labels/icons and keyboard-accessible inspection.

## F04 — Presentation language and subject breadth

The interface, instructional framing, navigation, explanations and feedback are exclusively English. This does not forbid quotations, original-language passages, vocabulary or dialogue exercises in another language. Preserve originals with English framing and clearly label translations.

Distinguish published translations, user translations and AI-generated translations. Do not silently replace an original with an English rendering. Right-to-left text and mixed-language documents need dedicated rendering tests.

Do not invent separate engines solely from the subject examples. Mathematics requires precise domains and assumptions; interpretive work requires author/speaker, context, edition, genre and interpretation. The same general model should accommodate both.

## F05 — JSON document format and structured content

**Accepted decision:** use JSON as the document format. MyST, custom Markdown file syntax and Logseq compatibility are not current implementation goals. Logseq's stable block/reference ideas are inspiration, not a chosen dependency.

JSON describes the document structure. SQLite handles persistence/history. They are complementary, not three separate chat/document/graph databases and not two independent authorities that may silently disagree.

The first content types are:

| Type | Required meaning | Viewer behavior |
| --- | --- | --- |
| Section | Heading and ordered contained content | Establish reading hierarchy and navigation. |
| Explanation | Narrative content in supported Markdown/LaTeX | Render prose, lists, quotations, examples and math naturally. |
| Claim | Statement plus scope, assumptions and exact identity/version | Show an identifiable statement; expose assumptions without pretending verification. |
| Reasoning | Argument/proof text with explicit claim-version links | Show how the text relates to statements, while distinguishing display from evaluation. |

Citations may attach to any appropriate content. Standalone equation, quotation, example, vocabulary and exercise types are not mandatory initially; they can be represented within Explanation.

Use a versioned, documented schema. Reject unknown structural fields/types where the active schema requires closed records. Unknown format versions must produce a meaningful error, not partial data loss. Persistent IDs come from the backend. Human-readable headings are not identities.

Claim display should reference stored claim content rather than hold an independently editable duplicate. A reasoning block is not necessarily a machine-executable inference rule. Ordinary prose should not automatically turn every assertion into a graph node; the exact claim-extraction workflow remains undecided.

The current Rust snapshot schema is an experimental subset, not the final four-type viewer contract. Example JSON in conversation was illustrative; it must not be copied as an authoritative API.

## F06 — Read-only document and strict mutation boundary

The learner cannot edit document prose, internal IDs or relationships directly in the viewer. Changes originate through chat or graph controls. External editors can alter exported files; the application cannot prevent that, but it must validate and reconcile their contents before accepting changes.

The AI must propose operations using a defined schema. It must not generate an arbitrary authoritative file. The service validates the proposal and generates the resulting document state.

An intended operation lifecycle:
1. Record the document and knowledge versions used for the request.
2. Receive a complete structured proposal; streamed fragments remain provisional.
3. Validate record shape, references, permissions, limits and structural invariants.
4. Construct a candidate and compute relevant changes/impact.
5. Obtain any required concrete learner authorization.
6. Recheck versions and commit the whole batch atomically.
7. Render the new state and schedule derived work.

Structural validity does not establish truth. Text inside explanation/code/quotation fields is content, never an executable instruction or a way to introduce IDs, approval flags or graph edges.

Reject malformed operations, nonexistent references, wrong object kinds, cross-workspace access, stale versions and unauthorized assessments. A batch must either apply completely or leave committed state unchanged. Retrying an acknowledged operation must not create duplicate claims.

Exactly which routine edits auto-apply and which require a diff remains open (D02). Manual source promotion and verification are already explicit learner actions; do not infer permission from AI-written approval fields.

## F07 — Document creation and organization

Eventually create independent documents for arbitrary learning goals. Possible starts include a blank document, existing notes/reference material or an AI-proposed outline. These entry options are proposed workflows, not three required first-viewer screens.

Documents should retain their discussions and source scope. Knowledge reuse should link an existing claim version, not duplicate it as another independent witness.

Users need a way to list, open, rename and eventually archive documents. Nested sections, content placement/reordering and removals require validated operations. Removing a displayed claim block should not erase the underlying historical claim, its assessments or scenario uses.

Creation is partly implemented in the Rust library. Full library navigation, import and user-facing creation workflows are not built.

## F08 — Claim versioning, provenance and revision impact

A permanent identity distinguishes one knowledge item from its versions. A changed statement, domain, assumption or interpretation can change the claim. Similar wording is not sufficient evidence of equivalence.

Example: “for positive integers” changed to “for all integers” creates a new claim version. Its old proof/assessment still refers to the positive-integer version. Other documents and scenarios should not silently inherit a broader statement.

Pure display reordering need not create a new claim version. Copying a display of the same version differs from creating a distinct claim. Backend IDs must not be duplicated accidentally.

Keep precise original passages and versions available when used as evidence. Preserve attribution: source assertion, quoted speaker, learner interpretation and AI reconstruction are different origins.

A changed claim flags relevant derivations for review. It does not automatically make every dependent conclusion false: another derivation may still justify that conclusion. Current code creates review-required markers but does not traverse a dependency graph.

## F09 — Revision history, storage growth and recovery

Saving current work and retaining every intermediate state are separate responsibilities. The user's concern is unnecessary writes and bloat; do not implement per-token or per-keystroke permanent snapshots.

Design direction:
- Replaceable recovery drafts for chat/forms/proposals.
- Meaningful committed states for accepted operations and explicit checkpoints.
- Shared unchanged content rather than repeatedly copying all claim text.
- Retention/thinning of unreferenced routine history.
- Protection for versions referenced by citations, assessments, curated results or scenarios.
- Inspectable storage usage and a preview of cleanup consequences.

Exact retention periods, backup cadence and checkpoint defaults are unresolved. Current SQLite code retains manifests, versions and receipts indefinitely; it does not satisfy the full bounded-storage requirement.

The viewer should eventually distinguish committed state, unsaved/proposed state and failed saves. Recovery must not falsely acknowledge a transaction that failed. Reading an older revision is different from restoring it; restore should create a new current revision with provenance rather than rewrite history. Diff/undo/restore interfaces remain planned.

## F10 — Sources and manual curation

The learner explicitly chooses when a useful result becomes part of the curated source. “Add to source” should present the exact statement, assumptions, reasoning, evidence and intended destination for review.

Example: after developing a proof in conversation, the learner can promote the theorem/proof result and separately record a verification assessment. Inclusion alone must not mark it verified.

Keep these distinctions:
- Chat is working discussion.
- Authored documents are developing learning resources.
- Imported references are source material with their own provenance.
- Curated claims are intentionally included knowledge.
- Assessments record judgments, separately from inclusion.

The source collection organization (one collection, multiple named collections, document-private collections) remains open. Cross-document reuse must respect explicit source scope.

## F11 — Trust and verification

New claims and relationships default to unverified in the assessment model. Unverified means not assessed, not false. Ordinary Study mode hides assessment bookkeeping; it does not mark the material true or delete stored judgments.

Use contextual assessments rather than one universal numeric trust score. A record should identify exact target version, judgment, method, scope, reviewer, date, rationale and evidence. Suggested labels include unverified, supported, verified within scope, disputed and refuted within scope; exact UI terminology still needs review.

“Verified by me” is not the same as machine-checked formal proof. Mathematical proof review, empirical evidence, textual interpretation and translation comparison have different standards. AI agreement, repetition, popularity and author prestige cannot automatically upgrade a judgment.

Retain multiple scoped assessments and disagreements. Do not make “latest timestamp wins” the general truth rule. Preserve superseded assessments on their original targets.

A philosophy book with factual errors can remain valuable. Challenge specific passages while retaining its arguments and historical/pedagogical value. Provenance, reliability, curation, learning value and hypothetical acceptance are separate attributes.

## F12 — Corrections, objections and counterexamples

Challenges target exact statements and versions and require their own evaluation. A common claim debunked by stronger evidence should remain discoverable with its correction and scope, not disappear from history.

For mathematics:
- A counterexample must satisfy every hypothesis and violate the conclusion.
- A proof gap invalidates or weakens that derivation; it does not alone refute the theorem.
- A changed definition/domain can invalidate a proposed counterexample.
- Preserve the original paper/passage and the challenge.
- Notify impacted derivations for review without propagating falsehood mechanically.

For interpretive/empirical work, establish that the evidence concerns the same event, sense, context or proposition. Competing interpretations should not be mislabeled as formal contradictions without an explicit mapping.

A focused counterexample workspace is a later UI idea; these semantic distinctions apply even before such a screen exists.

## F13 — Knowledge graph and relationship types

The graph connects claims, rules, evidence and source passages. It is not limited to a tree: claims can share premises, have multiple derivations, be cited by many documents and participate in cycles.

Distinguish derivation/dependency, evidential support, challenge/refutation, interpretation, citation and scoped equivalence. A line labeled “supports” is not permission to perform logical entailment.

Represent “A AND B implies C” as a rule requiring both premises. Two independent A→C and B→C arrows would change the meaning and are unacceptable.

AI can propose relationships; they remain unverified/provisional until the relevant explicit action. A source can be imported and read before its graph is extracted. Claims and relationships must preserve attribution, speaker, time, genre, edition and interpretation where relevant.

The graph inspector should explain an object's statement, assumptions, evidence, provenance, version and connections. Direct graph actions must use the same validation boundary as chat proposals.

## F14 — Hypothetical acceptance and logical evaluation

Scenarios let the learner temporarily accept claims and selected rules to explore consequences without altering permanent assessments.

Example: hypothetically accept claims from a biblical book, then inspect other books for derived agreement, explicit contradiction or unresolved relationships. This requires a reviewed extraction and interpretation; the application must not pretend that every passage has a single machine-readable meaning.

| Scenario display | Meaning |
| --- | --- |
| Blue / assumed | Explicit starting assumption, even if later conflicted. |
| Green / derived | Positive claim follows through the admitted rules. |
| Red / negated | Explicit negation follows through the admitted rules. |
| Gray / unresolved | Neither direction established in the evaluated scope. |
| Amber / conflict or provisional | Distinguish contradictory derivations from provisional interpretation with separate labels. |

These are conditional results, not permanent global truth labels. Compatibility is not entailment. Missing evidence is not refutation. A whole book can contain mixed outcomes; show coverage and counts rather than one green/red badge.

Track positive and negative derivations separately. If both P and not-P are derived, display both traces and contain the conflict; do not infer arbitrary unrelated conclusions. Cycles cannot establish their own starting premises. Converse/contrapositive reasoning is not automatic; the admitted formalism must explicitly allow any such rule.

Use bounded evaluation and disclose incomplete runs. No unrestricted theorem prover or exhaustive scholarly adjudication is promised.

## F15 — Scenario versions, comparison and conditional results

A saved scenario pins exact claims, rules, interpretation and evaluation configuration. Source updates do not silently rewrite it. An explicit refresh should show what changed and which results are affected.

Each derived result should expose premises, rule, conditions, source versions and trace. Compare scenarios and trace differences to premises, definitions, interpretations or evidence. Side-by-side presentation is a proposed UI, not an implemented screen.

A conditional result can be retained or curated without accepting its premises factually. A verified conditional relationship must still display its conditions. Accepting a whole-book claim set must preserve extraction coverage and its exact membership/version.

## F16 — Study and Critical review

Study is default: explain selected material, summarize, give examples and support understanding without requiring trust labels, hypothesis setup or graph completeness.

Critical review is an optional toggle that reveals assessments, challenges, dependencies and hypothetical tools. Turning it off hides these tools without changing stored values.

Taking a document as the learning material is not declaring every claim factual. Explicit requests to check/correct something must be handled honestly; contradictions that prevent coherent explanation should be acknowledged.

There is no dedicated Exam practice mode. Exercises can appear in explanations or be requested conversationally. Do not restore the removed Study mode badge when Study is the only ordinary mode.

## F17 — AI-guided authoring and provider integration

DeepSeek is the selected initial provider, with a replaceable adapter rather than provider-owned documents/state. The earlier prototype used the requested flash-model direction; exact current API model IDs must be verified when implementation resumes. Do not assume a rolling alias permanently identifies one snapshot or silently substitute a model.

The AI should support explanation, questions, comparisons, examples, argument development and proposing structured revisions. The application owns the document, conversations, references, assessments and history.

Require bounded requests, cancellation, errors that preserve committed work, usage reporting and recorded model/context provenance. A partial or failed response must not become a partial committed document.

The user must be able to understand what context leaves the computer. Exact per-source sending restrictions and approval defaults remain open. API keys do not belong in JSON documents, prompts, exports, logs or source control.

## F18 — Conversations and document collaboration

Conversations are ongoing work associated with documents. Preserve which user request produced an AI proposal, its base document version and the evidence/model used. Conversation edits/replacements should retain enough provenance to explain derived document changes.

Multiple threads per document are a design proposal. The required behavior is that switching documents does not confuse their conversation, context or source scope.

A conversation is not automatically a source. A summary derived from one book is not another independent witness. A learner may deliberately promote a result, but this requires the explicit curation workflow.

When chat and graph actions overlap, hold stale proposals for review. Never override a newer graph edit because an AI response finishes later.

## F19 — Reference ingestion and evidence preservation

Plan for Markdown, plain text and text-based PDFs as source inputs, alongside authored JSON documents. PDF support is ingestion/evidence inspection, not a return to the removed PDF-reader-first interface.

Preserve original bytes, title, author, edition, translation, language and locations. Keep printed page labels distinct from PDF file page indices. Extract structural boundaries where feasible: headings, definitions, theorem assumptions, proofs, paragraphs, quotations and examples.

Retain uncertainty about equations, reading order, tables, footnotes and damaged extraction. Source versions and extraction versions are separate. Re-extraction must not silently redirect old evidence spans.

Scanned OCR and specialized extraction are later features. Unsupported scans should be identified, not silently presented as fully indexed.

## F20 — Deep RAG and source scope

Retrieval runs locally and sends only selected evidence to the conversational provider. It should combine:
- Lexical search for exact terms, names, quotations and mathematical notation.
- Multilingual semantic retrieval for differently worded related ideas.
- Typed graph expansion for definitions, dependencies, corrections and interpretations.

Use explicit source scope per document. Expand proof fragments to their assumptions/theorem and textual passages to relevant context. Follow-up searches may fill missing definitions/evidence, with budgets for rounds, candidates, graph depth and tokens.

“Deep” means bounded targeted retrieval, not unlimited autonomous research. Graph extraction is not a prerequisite for ordinary reading. Similarity and model confidence are not inference rules.

Keep indexed versions/models explicit and indexes rebuildable. Choose embeddings based on actual hardware, languages, licenses and evaluation; do not assume a GPU. Embeddings and the conversational model are separate components.

## F21 — Citation inspection, lineage and retrieval honesty

Citations resolve to exact source versions and passages with surrounding context. Mark quotation, paraphrase, synthesis and conditional inference distinctly. Reject invented citation IDs, unsupported page references and fabricated quotations.

Track common source lineage. Multiple summaries of one passage must not appear as independent corroboration. A stored correction should be discoverable with the claim it challenges.

Show distinctions among imported, extracted, indexed, searched and assessed coverage. “No contradiction retrieved” is not “no contradiction exists.” Missing or stale evidence must be visible. A generated translation should remain linked to its original and be labeled as generated.

Retrieval evaluation should include known-answer passages, deliberately unanswerable questions, mixed languages, omitted assumptions, conflicting/corrected sources, duplicate summaries, ambiguous terms, out-of-scope answers and broken extraction. Measure relevance, citation correctness, context completeness, latency, memory and token use; set thresholds after a baseline.

## F22 — Persistence, portability and backups

One local SQLite database currently persists documents, revision manifests, immutable claim versions and accepted operation receipts. It does not yet persist all conversations, evidence, assessments or graph rules.

The target workspace must keep stable identities across restarts and OS migration. JSON is the document exchange/structure format; internal SQLite rows may normalize it. Imported originals are managed assets; copying rather than externally linking them is a technical proposal, not a settled requirement.

Backups must be consistent across metadata and referenced assets. Exclude credentials. Do not recommend copying an open database as if that automatically produced a complete backup. Restoration must verify integrity and report missing references. Cross-OS restoration needs tests before claiming portability.

Recovery must retain acknowledged commits, reject stale writes, avoid partial batches and support safe retries. Atomic SQLite saves are partly implemented; full workspace backup, publication and pruning are not.

## F23 — Credential handling and security audit

Use OS credential storage as the proposed persistent secret mechanism. Handle unavailable/locked Linux services explicitly; session-only fallback is recommended but awaits owner confirmation. Allow key replacement/deletion. Localhost alone is not authentication.

The explicit security-audit TODO includes local binding, session authorization, Host/Origin checks, CSRF/DNS rebinding, filesystem path boundaries, unsafe imports, rendering, source prompt injection, secret leakage, dependencies and update provenance.

No completed security audit is claimed. Current parsing/resource tests and database transactions are useful engineering checks, not certification. Do not label the app secure merely because it runs locally.

## F24 — Export, import and external modification

The current format direction is JSON only. Design document export/import to preserve typed content and exact relationships. A portable document with cross-document references must either include required dependencies or disclose unresolved ones.

Whether to offer a single self-contained JSON file, a JSON-plus-assets bundle or both remains open. Full workspace export must cover history/conversations/assessments separately from a reading-only document export.

External tools can edit JSON. Treat imported changes as proposals/candidates, validate structure and references, and do not authenticate claimed verification from arbitrary metadata. No silent overwrite of the live workspace.

MyST, custom Markdown syntax and Logseq import/export have been removed from the current work plan. Publication-oriented PDF/LaTeX export remains an optional later idea, not a prerequisite for the viewer.

## F25 — Testing, accessibility and operational quality

Tests must distinguish backend correctness, browser rendering, live-provider behavior, cross-OS portability and security auditing. A passing unit suite does not prove the others.

Viewer acceptance should cover Unicode/RTL quotations, math, long documents, keyboard use, clear navigation, contrast and textual equivalents of graph colors. Validate that malformed content cannot execute code or alter the document.

Storage tests should cover restart, conflicting writers, failures late in a transaction, interrupted transactions, disk-full conditions and recovery. Existing tests cover a subset; physical power loss and backup restoration remain separate.

Keep contributor setup reproducible with pinned toolchain/dependencies and documented commands. Packaging/updates, exact browser support and release criteria require further work.

## F26 — Optional future learning aids

Retain these as possibilities, not first-version promises:
- Open-question tracking and learning checkpoints.
- Progress/review scheduling that distinguishes reading completion from demonstrated understanding.
- Definition/terminology views across authors, notation and languages without assuming equivalence.
- Dedicated counterexample and claim-inspection workspaces.
- Formal proof-assistant integration.
- Advanced OCR/equation extraction.
- Publication-oriented export.

These ideas do not reintroduce Exam practice mode or mandatory subject templates.

## F27 — Optional external agent integration

A future local API/MCP adapter may let Codex, Claude Code or other tools inspect knowledge and propose changes. It must use the same schemas, version checks, source scopes and authorization boundaries.

External tools cannot bypass manual promotion, verification or hypothetical-rule admission simply because they are coding agents. Keep this optional; no current connector or runtime integration is implemented.

## F28 — Contributor continuation and non-goals

Preserve the owner's latest decisions, even when earlier documents/prototypes differ. Do not implement all suggestions as if they were individually approved. Register proposed behavior and unresolved choices explicitly.

Not current requirements:
- Cloud synchronization or multi-user collaboration.
- A PDF reader as the primary interface.
- Direct document editing.
- Whole-book true/false judgments.
- Automatic verification from AI confidence or source prestige.
- Unrestricted theorem proving.
- A MyST/Logseq/custom-Markdown document file format.
- A dedicated exam mode.

Contributors should first review the feature/decision registers with Rami, then agree on a viewer milestone and assign work. This handoff intentionally changes documentation only.
