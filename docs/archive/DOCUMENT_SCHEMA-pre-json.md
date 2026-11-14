# Structured document contract v1 — design draft

> HISTORICAL DRAFT — superseded. Preserved for rationale only. In particular, Markdown file authority, direct editing and the earlier release sequence are obsolete. Read [current decisions](../DECISIONS.md), [features](../FEATURES.md) and [the current version of this note](../DOCUMENT_SCHEMA.md).


Date: 14 November 2026. Accepted product boundary: the document pane is read-only; all authored changes originate in chat or graph actions. Proposed technical contract below is not implemented validation code. This supersedes editable-document and arbitrary Markdown-replacement proposals.

## Authority and portability

One workspace uses one SQLite database, not separate chat/document/graph databases. The backend commits a typed document model and its graph records together, then deterministically generates portable Markdown containing document-linked structure. Markdown is a serialization of that same state, not a second independently writable authority.

The current document-linked graph must be recoverable from a complete exported Markdown collection with its declared dependencies. Chat, assessment history and saved scenarios require a workspace export. A single file need not contain the entire workspace graph; it must explicitly list cross-document dependencies. Historical pinned claims must be included in the export dependency closure or reported missing.

Logseq-style stable block identities and references are a design influence. No decision to adopt the Logseq application, its storage engine or exact syntax has been made. The final Markdown encoding remains open; it must have a published specification and permissively licensed parser/serializer implementation. Do not claim interoperability before round-trip tests.

## Typed model

The notation below defines closed records: every listed field is required, nullable fields explicitly say null, and unlisted fields are rejected. Arrays may be empty unless constrained below. IDs are backend-generated opaque UUIDs. No caller may supply actor, verification status, timestamps or committed revision IDs as authoritative values.

- DocumentSnapshot: format_version (= 1), document_id, revision_id, title, root_section_ids[], sections[], blocks[], owned_knowledge_versions[], dependency_refs[].
- Section: id, title, parent_section_id (ID or null), ordered_items[].
- ItemRef: kind (section or block), id.
- NarrativeBlock: id, kind (= narrative), role (explanation/example/exercise/vocabulary/question/note), body_markdown, citations[].
- ClaimBlock: id, kind (= claim), claim_ref.
- RuleBlock: id, kind (= rule), rule_ref.
- ClaimVersion: item_id, version_id, kind (= claim), statement_markdown, language, scope_markdown, assumptions[], interpretation_markdown.
- Assumption: label, text_markdown. Labels unique within the version. These textual qualifications do not become graph premises automatically.
- RelationshipVersion: item_id, version_id, kind (= relationship), relation (supports/challenges/interprets/cites/equivalent_under_scope), from_claim, to_claim, scope_markdown, explanation_markdown.
- RuleVersion: item_id, version_id, kind (= rule), premises[], conclusion, conditions_markdown, explanation_markdown, formalism (= signed_forward_v1).
- SignedClaimRef: claim_ref, polarity (positive or negative).
- VersionRef: owner_document_id, item_id, version_id.
- Citation: local_label, evidence_span_id, use (quotation/paraphrase/background).
- DependencyRef: owner_document_id, item_id, version_id, content_hash.

claim_ref, rule_ref, from_claim and to_claim use VersionRef. Rule premises and conclusion use SignedClaimRef. owned_knowledge_versions is a discriminated union of ClaimVersion, RelationshipVersion and RuleVersion. blocks is a union of the three block types. An object has exactly one owning document; other documents reference its version. Ownership is storage responsibility, not a truth or authorship judgment.

A document may contain only narrative; graph creation is not a prerequisite for studying. Narrative that states a proposition does not silently create a claim.

## Structural invariants

1. Root sections have null parents. Every section/block occurs in exactly one place in the document tree. No orphan items, duplicate IDs, cycles or multiple parents.
2. Item ordering lives solely in ordered_items (and root_section_ids at root); parent pointers must agree. Maximum nesting is enforced.
3. Claim/rule blocks display the referenced version's content. They do not store a second independently editable copy of the statement.
4. Foreign references resolve to exact owned/dependency versions in the workspace. Cross-workspace references require explicit import.
5. Every knowledge version's item kind and owner agree with its stable identity. Versions and evidence are immutable.
6. A rule has at least one premise and exactly one conclusion. Duplicate signed premises are rejected. All premises are required.
7. Rules may participate in a graph cycle, but cycles do not bootstrap conclusions. Admission/evaluation is separate from representation.
8. signed_forward_v1 is a proposed finite, explicit signed-claim forward-rule representation. It performs no quantifier instantiation, converse, contraposition or automatic theorem proving. Narrative proof text does not establish formal validity.
9. Conditions expressed only in prose make a rule ineligible for automatic derivation until explicitly admitted within a scenario with the interpretive scope recorded. Representation does not imply verification.
10. Citation labels resolve through registered evidence spans; model-generated URLs/page numbers are not accepted as authoritative evidence.
11. Referenced identities are stable through moves. Copies create new block IDs; copying a claim block references the same version unless a distinct claim is deliberately created.
12. New knowledge is unverified and uncurated. Neither schema validity nor presence in the document upgrades trust.

## Content grammar and rendering

Content fields allow prose, emphasis, lists, tables, code examples, quotations and LaTeX math. No executable HTML/JavaScript, embedded components or structural directives are executed from these fields. Renderers must sanitize links and markup and disable unsafe math extensions.

A fenced code example may contain text resembling our file syntax; it remains literal content. The serializer must distinguish/escape content from generated metadata so that closing delimiters, IDs, comments or directives inside prose cannot create nodes or modify relationships. Parsing uses structure-aware tokenization, never global regular-expression extraction from arbitrary prose.

Presentation is English. Original-language passages and exercises are allowed with English framing. Language detection cannot prove compliance; provide a lint/review mechanism rather than corrupting quotations automatically.

Claim statements, scope, assumptions and interpretation are claim content. Any accepted change creates a new version—even a presumed spelling correction. A pure layout move changes the document revision but not the claim version.

## Serialization requirements before implementation

Choose the exact on-disk grammar after validating small examples. Requirements:
- Deterministic UTF-8 output with a format version and safe escaping.
- Human-readable Markdown without needing Living Source; unsupported metadata may be visible.
- Stable IDs, explicit object boundaries, exact version references, dependency manifest and preserved math.
- parse(serialize(model)) reproduces the semantic model; serialize(parse(canonical_file)) is byte-stable for the same format version.
- Unknown structural fields/versions yield an unsupported-format error, never silent omission.
- Duplicate IDs, malformed metadata and tampered references cannot overwrite trusted workspace state.
- Imported claims of verification are historical assertions to review, not authenticated local assessments.
- Hashes establish integrity against expected content, not authorship or trust.
- Ordinary imported Markdown is reference material or a candidate for structured conversion; it is never executed as a command.

No universal Markdown dialect guarantees our graph semantics. Publish both the format and import/export fixtures when choosing the grammar.

## Suggested initial limits

Proposed configurable implementation defaults: 1 MiB per proposal, 100 operations per batch, 64 KiB per content field, 10,000 blocks per document and section depth 16. Enforce UTF-8 byte limits, finite arrays and parsing time budgets. Test and adjust using real documents; these are resource limits, not learning-topic restrictions.

See [editing operations](../EDIT_OPERATIONS.md) for mutation and authorization, and [revision lifecycle](../DOCUMENT_REVISIONS.md) for storage/recovery.
