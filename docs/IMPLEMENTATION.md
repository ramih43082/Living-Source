# First implementation: document core

This is a runnable Rust library and command-line example, not the local browser application. The existing hosted UI has not been imported. No API key is needed and the engine makes no network requests.

## Run

Install Rust using rustup. The repository pins Rust 1.100.0 and includes Cargo.lock for reproducible dependency resolution. SQLite is bundled and compiled from source; a C toolchain is required (Linux compiler tools or Windows MSVC build tools). Run from the repository root:

```sh
cargo test --workspace --locked
cargo run --locked -p living-source-core --example demo
cargo run --locked -p living-source-core --example persistent_demo -- workspace.sqlite
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The demo consumes fixtures/create-claim.json, creates a section and claim and prints the committed structured document with backend-generated UUIDs. The original demo remains in-memory. persistent_demo creates a database and example document on first use, then reloads it on later runs. See [storage implementation](STORAGE.md) for durability boundaries.

## Implemented boundary

- Strict Serde records and tagged operations reject unknown fields and operation names.
- Six operations: rename_document, add_section, create_claim, place_claim, revise_claim, select_claim_version.
- Atomic candidate application, expected document/claim versions, no-op title handling and idempotency receipts (durable when using Store).
- Backend UUID generation, backward-only batch handles and kind-checked references.
- Immutable prior claim content with explicit display selection and review-required markers.
- Bounded proposal/field sizes, operation counts and document item counts.
- Generated JSON Schemas and a runnable example fixture.
- SQLite document/revision persistence, immutable claim bodies, durable proposal/retry records and initial schema migration.

The Rust engine is the authority for semantic checks. JSON Schema alone does not validate ownership, referenced versions, byte limits, resource budgets or logical truth. UTF-8 limits count bytes, not JSON Schema character lengths. Schema generation must not be presented as provider authorization or verification.

## Experimental wire profile

The code implements a deliberately smaller experimental profile than docs/EDIT_OPERATIONS.md. Do not treat protocol_version=1 as a frozen public API yet.

- Sections are root-only and append-only; add_section currently accepts op_id, new_handle and title only.
- Placement appends within a section. Nested sections, arbitrary positioning and narrative blocks are not implemented.
- References use explicit tagged objects: {"kind":"local","handle":"new:claim"} or {"kind":"existing","reference":{...}}. Section references use existing id instead of reference.
- place_claim uses claim; select_claim_version uses replacement. These are the actual executable field names.
- select_claim_version retains the same claim identity; replacing a display with a different claim will need a separate reviewed action.
- Cross-document references, rules, assessments, source promotion and scenarios are unsupported and cannot be injected through extra fields.

The broad design documents remain the target roadmap. The generated schemas describe the implemented subset; unsupported target operations fail closed. Stabilize naming and introduce explicit profile/version negotiation before connecting a provider or exposing a public API.

## Schema generation

```sh
cargo run --locked -p living-source-core --example schemas
```

Commit both generated files in schemas/. CI regenerates them and checks for drift. Snapshot schema describes output only; there is no trusted snapshot-import endpoint. Never instantiate live state by trusting an arbitrary deserialized snapshot.

## Trust boundary

Command is constructed by trusted application code and is intentionally not deserializable. It carries the expected document revision and idempotency key outside model output. apply is not an authentication or approval API. A future service must authenticate local commands, validate user authorization and display impact diffs before calling it. Nothing here can mark a claim verified.

Proposal parsing and candidate validation do not render HTML. Prose resembling metadata stays literal content. A renderer must still sanitize HTML/links/math; no XSS protection or security audit is claimed by these tests.

## Current continuation priority

The owner has paused implementation in favor of a detailed contributor handoff. Read [HANDOFF](HANDOFF.md), [FEATURES](FEATURES.md) and [DECISIONS](DECISIONS.md).

When coding resumes, the first usable milestone is the read-only JSON viewer with Section, Explanation, Claim and Reasoning. The implemented snapshot/proposal schemas are an experimental subset. Markdown/MyST serialization is no longer the next task; Markdown/LaTeX remain content within JSON fields.

Store now persists history and replay receipts. Claim text is not copied into each document checkpoint, but manifests and accepted proposal records still grow. Automatic pruning, backup/restore, encryption and publication are not implemented; do not claim bounded long-term storage yet.

CI is configured for Linux and Windows. A configuration is not proof of a passing remote run; inspect Actions results separately. Rust compilation/tests on the development Linux environment do not establish Windows packaging compatibility.
