# Current editing-operation boundary

The document is read-only. Future chat and graph controls use the same validated operations. This is separate from deciding which changes require review (D02) or which graph forms exist (D03).

## Implemented experimental profile

| Operation | Behavior |
| --- | --- |
| rename_document | Set title; unchanged title need not create a revision. |
| add_section | Append a root section. |
| create_claim | Create backend identity/version with statement, scope, assumptions, language and interpretation. |
| place_claim | Append a display referencing an exact claim version. |
| revise_claim | Create a new version and review marker; old references remain pinned. |
| select_claim_version | Explicitly change a block's displayed version of the same claim identity. |

Use [Rust types](../crates/document-core/src/lib.rs) and [generated schema](../schemas/proposal.schema.json) for actual field names. Unknown operations/fields fail. No assessment or source-promotion operation exists.

## Required future behavior

- Proposals are bounded closed records; complete streaming output is required before application.
- Trusted service context supplies identity, expected versions and authorization, not AI-generated flags.
- Validate ownership, references, source scope, shape, limits and invariants.
- Prepare an isolated candidate; show a concrete diff when required.
- Recheck versions and commit all operations atomically.
- Return consistent outcomes for acknowledged retries.
- Preserve provenance and schedule dependency review.
- Never interpret content strings as executable directives.
- Validation establishes structure, not truth.

Narrative/Reasoning editing, nesting/reordering/removal, relationships, rules, curation, assessments and scenarios are future command families. Source promotion/verification remain explicit learner actions.

The old [complete proposed operation catalog](archive/EDIT_OPERATIONS-pre-json.md) is retained for ideas only. It differs from current code and is not an API promise. Stabilize the four-type JSON viewer schema before extending it.
