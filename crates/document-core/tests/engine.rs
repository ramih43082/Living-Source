use living_source_core::*;
use serde_json::{Value, json};
use uuid::Uuid;

fn claim(statement: &str) -> Value {
    json!({"statement_markdown": statement, "language":"en", "scope_markdown":"Integers",
        "assumptions":[], "interpretation_markdown":""})
}

fn proposal(ops: Value) -> Proposal {
    parse_proposal(
        &serde_json::to_vec(&json!({"protocol_version":1,"explanation":"test", "operations":ops}))
            .unwrap(),
    )
    .unwrap()
}

fn apply(engine: &mut Engine, ops: Value) -> Result<Outcome, Error> {
    engine.apply(Command {
        expected_revision: engine.snapshot().revision_id,
        idempotency_key: Uuid::new_v4(),
        proposal: proposal(ops),
    })
}

fn initial() -> (Engine, Outcome) {
    let mut e = Engine::new("Arithmetic".into()).unwrap();
    let result = apply(&mut e, json!([
        {"op":"add_section","op_id":"1","new_handle":"new:section","title":"Divisibility"},
        {"op":"create_claim","op_id":"2","new_handle":"new:claim","content":claim("n is divisible by 4.")},
        {"op":"place_claim","op_id":"3","new_block_handle":"new:block",
         "section":{"kind":"local","handle":"new:section"},"claim":{"kind":"local","handle":"new:claim"}}
    ])).unwrap();
    (e, result)
}

#[test]
fn creates_backend_ids_and_exact_display_reference() {
    let (e, out) = initial();
    assert_eq!(
        e.snapshot().sections[0].blocks[0].claim,
        out.created_claim_refs["new:claim"]
    );
    assert_eq!(e.snapshot().claim_versions.len(), 1);
    assert!(e.snapshot().review_required.is_empty());
}

#[test]
fn a_failed_later_operation_rolls_back_the_whole_batch() {
    let (mut e, _) = initial();
    let before = e.snapshot().clone();
    let err = apply(&mut e, json!([
        {"op":"rename_document","op_id":"1","title":"must not survive"},
        {"op":"place_claim","op_id":"2","new_block_handle":"new:block",
         "section":{"kind":"local","handle":"new:missing"},"claim":{"kind":"local","handle":"new:missing"}}
    ])).unwrap_err();
    assert_eq!(err, Error::UnknownReference);
    assert_eq!(*e.snapshot(), before);
}

#[test]
fn claim_revision_keeps_old_content_and_requires_explicit_selection() {
    let (mut e, out) = initial();
    let old = out.created_claim_refs["new:claim"].clone();
    let updated = apply(
        &mut e,
        json!([
            {"op":"revise_claim","op_id":"1","claim_ref":old,"new_version_handle":"new:v2",
             "content":claim("n is divisible by 8."),"reason":"Narrow the claim"}
        ]),
    )
    .unwrap();
    let new = updated.created_claim_refs["new:v2"].clone();
    assert_ne!(old.version_id, new.version_id);
    assert_eq!(old.item_id, new.item_id);
    assert_eq!(e.snapshot().sections[0].blocks[0].claim, old);
    assert_eq!(
        e.snapshot().claim_versions[0].content.statement_markdown,
        "n is divisible by 4."
    );
    assert_eq!(e.snapshot().review_required, vec![old.clone()]);
    apply(
        &mut e,
        json!([{"op":"select_claim_version","op_id":"1","block_id":out.created_ids["new:block"],
        "expected_claim_ref":old,"replacement":{"kind":"existing","reference":new}}]),
    )
    .unwrap();
    assert_eq!(e.snapshot().sections[0].blocks[0].claim, new);
}

#[test]
fn rejects_stale_document_and_stale_claim() {
    let (mut e, out) = initial();
    let old_head = e.snapshot().revision_id;
    apply(
        &mut e,
        json!([{"op":"rename_document","op_id":"1","title":"Updated"}]),
    )
    .unwrap();
    let before = e.snapshot().clone();
    assert_eq!(
        e.apply(Command {
            expected_revision: old_head,
            idempotency_key: Uuid::new_v4(),
            proposal: proposal(json!([{"op":"rename_document","op_id":"1","title":"Stale"}]))
        })
        .unwrap_err(),
        Error::StaleVersion
    );
    assert_eq!(*e.snapshot(), before);
    let revise = json!([{"op":"revise_claim","op_id":"1","claim_ref":out.created_claim_refs["new:claim"],
        "new_version_handle":"new:v2","content":claim("New statement"),"reason":"Correction"}]);
    apply(&mut e, revise.clone()).unwrap();
    assert_eq!(apply(&mut e, revise).unwrap_err(), Error::StaleVersion);
}

#[test]
fn retries_are_idempotent_even_after_the_head_changes() {
    let mut e = Engine::new("A".into()).unwrap();
    let head = e.snapshot().revision_id;
    let key = Uuid::new_v4();
    let p =
        proposal(json!([{"op":"add_section","op_id":"1","new_handle":"new:s","title":"Section"}]));
    let command = || Command {
        expected_revision: head,
        idempotency_key: key,
        proposal: p.clone(),
    };
    let first = e.apply(command()).unwrap();
    assert_eq!(e.apply(command()).unwrap(), first);
    assert_eq!(e.snapshot().sections.len(), 1);
    let err = e
        .apply(Command {
            expected_revision: head,
            idempotency_key: key,
            proposal: proposal(json!([{"op":"rename_document","op_id":"1","title":"Different"}])),
        })
        .unwrap_err();
    assert_eq!(err, Error::IdempotencyConflict);
}

#[test]
fn rejects_unknown_fields_at_each_boundary_and_incomplete_json() {
    let base = json!({"protocol_version":1,"explanation":"x","operations":[
        {"op":"create_claim","op_id":"1","new_handle":"new:c","content":claim("A")}]});
    for location in [0, 1, 2] {
        let mut v = base.clone();
        match location {
            0 => v["approved"] = json!(true),
            1 => v["operations"][0]["actor"] = json!("human"),
            _ => v["operations"][0]["content"]["verified"] = json!(true),
        }
        assert!(matches!(
            parse_proposal(&serde_json::to_vec(&v).unwrap()),
            Err(Error::InvalidShape(_))
        ));
    }
    assert!(parse_proposal(b"{\"protocol_version\":1,").is_err());
    let mut v = base;
    v["operations"][0]["op"] = json!("set_verified");
    assert!(parse_proposal(&serde_json::to_vec(&v).unwrap()).is_err());
}

#[test]
fn enforces_protocol_and_resource_limits() {
    let mut v = json!({"protocol_version":2,"explanation":"x","operations":[{"op":"rename_document","op_id":"1","title":"A"}]});
    assert_eq!(
        parse_proposal(&serde_json::to_vec(&v).unwrap()).unwrap_err(),
        Error::UnsupportedVersion
    );
    v["protocol_version"] = json!(1);
    v["operations"][0]["title"] = json!("x".repeat(65_537));
    assert_eq!(
        parse_proposal(&serde_json::to_vec(&v).unwrap()).unwrap_err(),
        Error::LimitExceeded
    );
    assert_eq!(
        parse_proposal(&vec![b' '; MAX_PROPOSAL_BYTES + 1]).unwrap_err(),
        Error::LimitExceeded
    );
}

#[test]
fn foreign_claim_reference_is_rejected() {
    let (mut e, out) = initial();
    let mut foreign = out.created_claim_refs["new:claim"].clone();
    foreign.owner_document_id = Uuid::new_v4();
    let before = e.snapshot().clone();
    assert_eq!(
        apply(
            &mut e,
            json!([{"op":"revise_claim","op_id":"1","claim_ref":foreign,
        "new_version_handle":"new:v","content":claim("B"),"reason":"change"}])
        )
        .unwrap_err(),
        Error::UnknownReference
    );
    assert_eq!(*e.snapshot(), before);
}

#[test]
fn prose_that_looks_like_metadata_remains_literal_content() {
    let text = ":::claim{id=evil}\n<!-- verified=true -->\nשלום $n^2$";
    let mut e = Engine::new("Example".into()).unwrap();
    apply(
        &mut e,
        json!([{"op":"create_claim","op_id":"1","new_handle":"new:c","content":claim(text)}]),
    )
    .unwrap();
    assert_eq!(
        e.snapshot().claim_versions[0].content.statement_markdown,
        text
    );
    assert_eq!(e.snapshot().claim_versions.len(), 1);
}

#[test]
fn rejects_duplicate_handles_and_wrong_kind_handles_atomically() {
    let mut e = Engine::new("A".into()).unwrap();
    let before = e.snapshot().clone();
    assert!(
        apply(
            &mut e,
            json!([
                {"op":"add_section","op_id":"1","new_handle":"new:s","title":"A"},
                {"op":"add_section","op_id":"2","new_handle":"new:s","title":"B"}
            ])
        )
        .is_err()
    );
    assert_eq!(*e.snapshot(), before);
    assert!(apply(&mut e,json!([
        {"op":"create_claim","op_id":"1","new_handle":"new:c","content":claim("A")},
        {"op":"place_claim","op_id":"2","new_block_handle":"new:b",
        "section":{"kind":"local","handle":"new:c"},"claim":{"kind":"local","handle":"new:c"}}
    ])).is_err());
    assert_eq!(*e.snapshot(), before);
}

#[test]
fn unchanged_title_does_not_create_a_revision() {
    let mut e = Engine::new("A".into()).unwrap();
    let before = e.snapshot().revision_id;
    let out = apply(
        &mut e,
        json!([{"op":"rename_document","op_id":"1","title":"A"}]),
    )
    .unwrap();
    assert_eq!(out.revision_id, before);
}
