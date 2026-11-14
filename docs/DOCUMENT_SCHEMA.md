# Current document schema direction

Status: product contract direction, not a completed new JSON Schema. See [F05](FEATURES.md#f05--json-document-format-and-structured-content) and decisions D07/D15/D17.

## Accepted structure

JSON is the outer document format. The read-only viewer renders it. Markdown/LaTeX are permitted within designated content fields; they are not a separate authoritative document file.

| Type | Semantics | Current implementation gap |
| --- | --- | --- |
| Section | Heading plus ordered content | Current code has root-only sections with claim blocks. |
| Explanation | Prose, quotations, examples, lists, vocabulary/exercises and math | Not implemented. |
| Claim | Versioned statement, scope, assumptions and interpretation | Basic claim content/versioning exists. |
| Reasoning | Displayed argument/proof linked to exact claim versions | Not implemented; distinct from an executable rule. |

Citations attach to appropriate blocks. An equation can live inside Explanation; no dedicated equation type is required for the initial viewer.

## Required invariants

- Stable identities, explicit format version and exact references.
- Claim display references claim content; avoid two editable copies.
- Unknown/malformed structural data fails clearly rather than being silently discarded.
- IDs/relationships cannot be altered from the read-only viewer.
- Embedded content never executes structural commands or code.
- New or modified claim content does not inherit verification.
- A displayed reasoning block does not imply logical validity.
- Ordinary Markdown prose does not automatically create graph nodes.
- Unsupported imports cannot overwrite existing workspace state.

## Decisions still needed

Choose the exact field names/discriminators, nesting and order representation, citation objects, dependency packaging and schema evolution rules. The viewer's initial file-open versus stored-document workflow is also open.

The [existing snapshot schema](../schemas/snapshot.schema.json) and [proposal schema](../schemas/proposal.schema.json) describe the implemented experimental subset. They do not yet implement Explanation/Reasoning. Do not copy illustrative chat JSON as the final API.

The previous [pre-JSON draft](archive/DOCUMENT_SCHEMA-pre-json.md) is retained for history, not as an instruction to build Markdown serialization.
