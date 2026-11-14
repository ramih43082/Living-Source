use living_source_core::{
    Command, Error, parse_proposal,
    storage::{StorageError, Store},
};
use rusqlite::Connection;
use serde_json::{Value, json};
use uuid::Uuid;

fn command(head: Uuid, key: Uuid, operations: Value) -> Command {
    Command {
        expected_revision: head,
        idempotency_key: key,
        proposal: parse_proposal(
            &serde_json::to_vec(
                &json!({"protocol_version":1,"explanation":"test","operations":operations}),
            )
            .unwrap(),
        )
        .unwrap(),
    }
}
fn rename(head: Uuid, key: Uuid, title: &str) -> Command {
    command(
        head,
        key,
        json!([{"op":"rename_document","op_id":"1","title":title}]),
    )
}
fn content(text: &str) -> Value {
    json!({"statement_markdown":text,"language":"en","scope_markdown":"integers","assumptions":[],"interpretation_markdown":""})
}
fn count(db: &Connection, table: &str) -> i64 {
    // Only literal table names from tests call this helper.
    db.query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
        .unwrap()
}

#[test]
fn reopen_preserves_document_and_durable_retry() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.sqlite");
    let mut db = Store::open(&path).unwrap();
    let initial = db.create_document("First".into()).unwrap();
    let key = Uuid::new_v4();
    let out = db
        .apply(
            initial.document_id,
            rename(initial.revision_id, key, "שלום $x^2$"),
        )
        .unwrap();
    let expected = db.snapshot(initial.document_id).unwrap();
    drop(db);
    let mut db = Store::open(&path).unwrap();
    assert_eq!(db.snapshot(initial.document_id).unwrap(), expected);
    assert_eq!(
        db.apply(
            initial.document_id,
            rename(initial.revision_id, key, "שלום $x^2$")
        )
        .unwrap(),
        out
    );
    assert!(matches!(
        db.apply(
            initial.document_id,
            rename(initial.revision_id, key, "Other")
        ),
        Err(StorageError::Engine(Error::IdempotencyConflict))
    ));
    assert_eq!(
        db.revision(initial.document_id, initial.revision_id)
            .unwrap(),
        initial
    );
    assert_eq!(db.document_ids().unwrap(), vec![initial.document_id]);
}

#[test]
fn two_connections_reject_stale_writes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.sqlite");
    let mut a = Store::open(&path).unwrap();
    let initial = a.create_document("A".into()).unwrap();
    let mut b = Store::open(&path).unwrap();
    a.apply(
        initial.document_id,
        rename(initial.revision_id, Uuid::new_v4(), "B"),
    )
    .unwrap();
    assert!(matches!(
        b.apply(
            initial.document_id,
            rename(initial.revision_id, Uuid::new_v4(), "C")
        ),
        Err(StorageError::Engine(Error::StaleVersion))
    ));
    assert_eq!(b.snapshot(initial.document_id).unwrap().title, "B");
}

#[test]
fn late_failure_rolls_back_claim_head_revision_and_receipt() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.sqlite");
    let mut db = Store::open(&path).unwrap();
    let initial = db.create_document("A".into()).unwrap();
    let raw = Connection::open(&path).unwrap();
    raw.execute_batch("CREATE TRIGGER fail_receipt BEFORE INSERT ON receipts BEGIN SELECT RAISE(ABORT, 'injected failure'); END;").unwrap();
    let key = Uuid::new_v4();
    let change = || {
        command(
            initial.revision_id,
            key,
            json!([
                {"op":"rename_document","op_id":"1","title":"B"},
                {"op":"create_claim","op_id":"2","new_handle":"new:c","content":content("A claim")}
            ]),
        )
    };
    assert!(matches!(
        db.apply(initial.document_id, change()),
        Err(StorageError::Sqlite(_))
    ));
    assert_eq!(db.snapshot(initial.document_id).unwrap(), initial);
    assert_eq!(count(&raw, "revisions"), 1);
    assert_eq!(count(&raw, "claim_versions"), 0);
    assert_eq!(count(&raw, "receipts"), 0);
    raw.execute_batch("DROP TRIGGER fail_receipt").unwrap();
    db.apply(initial.document_id, change()).unwrap();
    assert_eq!(db.snapshot(initial.document_id).unwrap().title, "B");
}

#[test]
fn claim_history_preserves_old_content_without_copying_bodies_per_revision() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.sqlite");
    let mut db = Store::open(&path).unwrap();
    let initial = db.create_document("A".into()).unwrap();
    let out=db.apply(initial.document_id,command(initial.revision_id,Uuid::new_v4(),json!([
        {"op":"create_claim","op_id":"1","new_handle":"new:c","content":content("positive integers")}
    ]))).unwrap();
    let first = db.snapshot(initial.document_id).unwrap();
    let next=db.apply(initial.document_id,command(out.revision_id,Uuid::new_v4(),json!([
        {"op":"revise_claim","op_id":"1","claim_ref":out.created_claim_refs["new:c"],"new_version_handle":"new:v","content":content("all integers"),"reason":"Broaden domain"}
    ]))).unwrap();
    db.apply(
        initial.document_id,
        rename(next.revision_id, Uuid::new_v4(), "New title"),
    )
    .unwrap();
    drop(db);
    let mut db = Store::open(&path).unwrap();
    assert_eq!(
        db.revision(initial.document_id, first.revision_id).unwrap(),
        first
    );
    let current = db.snapshot(initial.document_id).unwrap();
    assert_eq!(current.claim_versions.len(), 2);
    assert_eq!(
        current.claim_versions[0].content.statement_markdown,
        "positive integers"
    );
    assert_eq!(
        current.review_required,
        vec![out.created_claim_refs["new:c"].clone()]
    );
    let raw = Connection::open(&path).unwrap();
    assert_eq!(count(&raw, "claim_versions"), 2);
    assert_eq!(raw.query_row("SELECT count(*) FROM revisions WHERE json_array_length(manifest, '$.claim_versions') != 0",[],|r|r.get::<_,i64>(0)).unwrap(),0);
    assert_eq!(raw.query_row("SELECT json_extract(proposal, '$.operations[0].reason') FROM receipts WHERE revision_id = ?1",[next.revision_id.to_string()],|r|r.get::<_,String>(0)).unwrap(),"Broaden domain");
}

#[test]
fn rejects_unrelated_and_future_databases() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("other.sqlite");
    let raw = Connection::open(&path).unwrap();
    raw.execute_batch("CREATE TABLE unrelated(value TEXT); INSERT INTO unrelated VALUES ('keep');")
        .unwrap();
    assert!(matches!(
        Store::open(&path),
        Err(StorageError::IncompatibleDatabase)
    ));
    assert_eq!(
        raw.query_row("SELECT value FROM unrelated", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "keep"
    );
    let future = dir.path().join("future.sqlite");
    drop(Store::open(&future).unwrap());
    let raw = Connection::open(&future).unwrap();
    raw.pragma_update(None, "user_version", 99).unwrap();
    assert!(matches!(
        Store::open(&future),
        Err(StorageError::IncompatibleDatabase)
    ));
    assert_eq!(
        raw.pragma_query_value(None, "user_version", |r| r.get::<_, i32>(0))
            .unwrap(),
        99
    );
}

#[test]
fn no_op_keeps_revision_and_keys_are_workspace_scoped() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.sqlite");
    let mut db = Store::open(&path).unwrap();
    let a = db.create_document("A".into()).unwrap();
    let b = db.create_document("B".into()).unwrap();
    let key = Uuid::new_v4();
    assert_eq!(
        db.apply(a.document_id, rename(a.revision_id, key, "A"))
            .unwrap()
            .revision_id,
        a.revision_id
    );
    assert!(matches!(
        db.apply(b.document_id, rename(b.revision_id, key, "B")),
        Err(StorageError::Engine(Error::IdempotencyConflict))
    ));
    assert_eq!(count(&Connection::open(path).unwrap(), "revisions"), 2);
}

#[test]
#[ignore]
fn interrupted_writer() {
    let Ok(path) = std::env::var("LIVING_SOURCE_CRASH_TEST_DB") else {
        return;
    };
    let db = Connection::open(path).unwrap();
    db.execute_batch("BEGIN IMMEDIATE; UPDATE revisions SET manifest=json_set(manifest, '$.title', 'uncommitted');").unwrap();
    std::process::exit(0);
}

#[test]
fn interrupted_transaction_recovers_on_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("workspace.sqlite");
    let mut db = Store::open(&path).unwrap();
    let original = db.create_document("Keep".into()).unwrap();
    drop(db);
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "interrupted_writer"])
        .env("LIVING_SOURCE_CRASH_TEST_DB", &path)
        .output()
        .unwrap();
    assert!(result.status.success());
    let mut db = Store::open(&path).unwrap();
    assert_eq!(db.snapshot(original.document_id).unwrap(), original);
    assert_eq!(
        Connection::open(path)
            .unwrap()
            .query_row("PRAGMA integrity_check", [], |r| r.get::<_, String>(0))
            .unwrap(),
        "ok"
    );
}
