use living_source_core::{Command, parse_proposal, storage::Store};
use uuid::Uuid;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: persistent_demo <workspace.sqlite>")?;
    let mut db = Store::open(path)?;
    let ids = db.document_ids()?;
    let document = if let Some(id) = ids.first() {
        *id
    } else {
        let initial = db.create_document("Persistent learning document".into())?;
        db.apply(
            initial.document_id,
            Command {
                expected_revision: initial.revision_id,
                idempotency_key: Uuid::new_v4(),
                proposal: parse_proposal(include_bytes!("../../../fixtures/create-claim.json"))?,
            },
        )?;
        initial.document_id
    };
    println!("{}", serde_json::to_string_pretty(&db.snapshot(document)?)?);
    Ok(())
}
