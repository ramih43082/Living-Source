# Editing operation contract v1 — design draft

Status: specification, not a working API or security audit. The document pane offers reading, selection, copying, citation inspection and navigation only. Optional source view is read-only too. Graph forms allow deliberate content/structure edits without needing a model; chat translates requests to the same operations.

## Proposal envelope

A model returns only:
- protocol_version: 1
- operations: a nonempty ordered array of the closed operation records below
- explanation: a short user-facing explanation

The application wraps this in trusted context: proposal_id, workspace/document ID, expected document revision, expected version of each touched knowledge item, request idempotency key, originating user action and AI run identity. Values supplied by a model cannot override trusted context. Claim references from the proposal must resolve within the permitted context/source scope.

New objects use batch-local handles (for example new:claim1). Handles are unique, may refer only to earlier operations, expire with the batch and are never stored as persistent identities. Backend IDs are allocated only during application. Existing references use exact IDs and versions. There is no generic patch, arbitrary SQL, arbitrary file path, replace_document or set_verified operation.

## Allowed document and graph operations

All fields after the operation name are required unless explicitly nullable. Each operation also has a unique batch-local op_id. Unknown operations/fields are errors. Full content records use the types defined in [document schema](DOCUMENT_SCHEMA.md).

| Operation | Payload | Effect |
| --- | --- | --- |
| rename_document | title | Changes display title only. |
| add_section | new_handle, parent_section (ID/handle/null), before_item (ID/handle/null), title | Null placement appends; null parent means root. |
| rename_section | section_id, title | Preserves section identity. |
| add_narrative | new_handle, section, before_item, role, body_markdown, citations | Creates a content block without inventing a claim. |
| revise_narrative | block_id, role, body_markdown, citations | Replaces only that narrative block's content. |
| create_claim | new_handle, full claim content | Creates a new unverified, uncurated identity/version. |
| revise_claim | claim_ref, new_version_handle, full replacement claim content, reason | Creates a version; does not mutate old evidence or dependents. |
| place_claim | new_block_handle, section, before_item, claim_ref_or_handle | Inserts a display of that exact claim version. |
| select_claim_version | block_id, expected_claim_ref, replacement_claim_ref_or_handle | Deliberately changes what a claim block displays. |
| create_relationship | new_handle, full relationship content | Creates an unverified, versioned typed connection. |
| revise_relationship | relationship_ref, new_version_handle, full replacement content, reason | Preserves old relationship/version. |
| create_rule | new_handle, full rule content | Creates an unverified multi-premise rule. |
| revise_rule | rule_ref, new_version_handle, full replacement content, reason | Does not transfer old assessment or scenario admission. |
| place_rule | new_block_handle, section, before_item, rule_ref_or_handle | Displays an exact rule version. |
| select_rule_version | block_id, expected_rule_ref, replacement_rule_ref_or_handle | Explicitly updates a rule display. |
| move_item | item_id, destination_section (ID/null), before_item | Reorders/moves; only sections can be at root. |
| remove_block | block_id, reason | Removes display from this document; retains referenced knowledge/history. |
| remove_empty_section | section_id | Fails if the section still contains items. |
| detach_relationship | relationship_ref, reason | Removes the document's active relationship membership, preserving history/scenarios. |

Placement before an item requires it to be a child of the same destination. All operations target the envelope's owning document. Cross-document references are allowed, but cross-document writes are rejected in v1; prepare a separate proposal for the owner. All expected versions are checked even when a reference comes from another document.

Claim/rule content excludes backend-generated item/version IDs. Citation changes are part of revise_narrative or exact evidence bindings; adding evidence does not verify a claim. An owned knowledge item can exist without a display block, but must remain visible in the graph/knowledge inventory.

Revise_claim plus select_claim_version can be one atomic batch. Other blocks, relationships, rules and scenarios referencing the previous version remain pinned; affected items receive review tasks. Do not silently rewrite every reference to the new version.

Document creation, archive/restore, source import, assessment, curation and scenario changes are separate command families, not escape hatches for arbitrary content mutation.

## Explicit learner actions

The model can propose assessments, curation and scenario changes in conversation, but cannot authorize them by outputting a field such as approved: true.

Promotion, assessment recording/supersession, rule admission and scenario acceptance require a concrete learner action against the exact version and payload being approved. A graph action or a chat review card may supply it; do not demand a second confirmation after that concrete authorization. A general “write this section” request or enabled auto-update setting is not authorization to verify a claim.

Record the local action and payload hash in the backend; never trust identity/approval flags inside model output. Editing an approved payload invalidates the prior approval. No global confirmation is needed for every routine narrative change.

Suggested defaults: narrative additions/revisions can auto-apply when enabled and fresh; changes/removals to existing claims, rules or relationships show a concrete impact diff unless the learner already submitted those exact changes through graph controls.

## Validation and application

1. Parse a complete bounded response. Streaming fragments never mutate documents.
2. Validate protocol, discriminated record shapes, required fields, enum values, lengths and handle ordering. No silent “best effort” repairs.
3. Resolve object kinds, ownership, scope, evidence and immutable version references.
4. Apply operations to an isolated candidate model; check tree, graph and content invariants.
5. Compute a diff and dependency-review consequences. Verify any required learner authorization against this candidate.
6. Recheck expected document/item versions inside the write transaction.
7. Atomically commit new versions, head, operation result, review tasks and publication/index jobs. One invalid operation rejects the entire batch.
8. Return backend IDs for local handles, resulting revision and pending export/index status. Render committed state; publish the Markdown separately.

Retry with the same idempotency key and same request yields the same outcome. Same key with different input fails. A stale proposal goes to review; it is never automatically rebased onto newer claims. Regeneration/reconciliation creates a new proposal and a new reviewable diff.

Validation proves well-formedness and referential consistency, not mathematical or factual truth.

## Example proposal

The runtime supplies the document/base revision. This creates a claim and places its display in an existing section; IDs below are illustrative.

```json
{
  "protocol_version": 1,
  "explanation": "Add an explicit claim about divisibility.",
  "operations": [
    {
      "op_id": "op1",
      "op": "create_claim",
      "new_handle": "new:claim1",
      "content": {
        "statement_markdown": "The integer n is divisible by 4.",
        "language": "en",
        "scope_markdown": "Integer arithmetic",
        "assumptions": [],
        "interpretation_markdown": ""
      }
    },
    {
      "op_id": "op2",
      "op": "place_claim",
      "new_block_handle": "new:block1",
      "section": "existing-section-id",
      "before_item": null,
      "claim_ref_or_handle": "new:claim1"
    }
  ]
}
```

This does not promote or verify the claim. A schema-valid response containing a nonexistent section still fails backend validation.

## Errors and acceptance cases

Return code, operation ID, field path, readable reason and current revision where safe; no credentials or raw provider headers. Codes: INVALID_SHAPE, UNSUPPORTED_VERSION, LIMIT_EXCEEDED, UNKNOWN_REFERENCE, WRONG_KIND, WRONG_OWNER, OUT_OF_SCOPE, STALE_VERSION, INVALID_STRUCTURE, AUTHORIZATION_REQUIRED, IDEMPOTENCY_CONFLICT and STORAGE_FAILURE.

Required future tests:
- Reject unknown operations/fields, forged actor/approval fields and malformed nested records.
- Treat directive-looking text inside prose/code as literal content, never a command.
- Reject references to the wrong kind, missing evidence, foreign workspace or disallowed source.
- Reject tree cycles/duplicate placement and forward/duplicate local handles.
- Revise a claim: create a version, preserve old verification, flag dependencies.
- Reject stale batches and all-or-nothing batches with one invalid operation.
- Replay an acknowledged request: no duplicate objects/revisions.
- Cancel/fail streaming: committed state remains unchanged.
- Verify graph forms and AI proposals pass through the same validator.
- Editing/removing exported metadata cannot silently modify live graph state.

Next implementation deliverables: machine-readable JSON Schemas for all closed records, matching Rust types, provider adapter mapping, positive/negative fixtures and transactional tests. A provider's structured-output mode is helpful but never replaces backend validation.
