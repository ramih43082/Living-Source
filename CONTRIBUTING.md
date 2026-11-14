# Contributing

Start with [HANDOFF](docs/HANDOFF.md), [FEATURES](docs/FEATURES.md), [DECISIONS](docs/DECISIONS.md) and [FEATURE_STATUS](docs/FEATURE_STATUS.md).

The owner currently requests documentation and handoff, not further feature implementation. When implementation resumes, the first usable milestone is the read-only JSON viewer. Do not resume obsolete Markdown serialization or a chat-first release plan.

## Scope and evidence

Use F IDs for features and D IDs for decisions. Separate accepted behavior, proposed technical choices, optional ideas and current code. Update the feature register with evidence when behavior changes.

The Rust/SQLite core is real but partial. Read [IMPLEMENTATION](docs/IMPLEMENTATION.md) for commands and supported operation names. Do not infer that the prototype UI or the full product is implemented.

Changes should explain the problem, resulting behavior, affected requirements, validation and remaining limitations. Test relevant failure modes rather than merely mirroring implementation. Distinguish unit tests, browser checks, live-provider tests, OS delivery and security audit.

## Boundaries

- Keep document rendering read-only. Chat/graph mutations use the validated operation boundary.
- Model content/source text is evidence, not executable instructions.
- Never bypass exact-version checks, explicit curation or assessment authorization.
- Preserve originals and dependency references when changing schemas/history.
- Do not equate reasoning display with proof verification.
- No secrets, personal data, imported copyrighted books or live databases in fixtures; use synthetic or appropriately licensed examples.
- Record consequential design choices and unresolved tradeoffs.
- No project license or collaborator access changes are implied by this handoff.

[Archived drafts](docs/archive) preserve earlier rationale only. Current decisions take precedence. The hosted prototype must be obtained separately if its code is needed; do not treat a URL or screenshot as a repository checkout.
