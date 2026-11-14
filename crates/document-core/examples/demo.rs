use living_source_core::{Command, Engine, parse_proposal};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Engine::new("First learning document".into())?;
    let proposal = parse_proposal(include_bytes!("../../../fixtures/create-claim.json"))?;
    let result = engine.apply(Command {
        expected_revision: engine.snapshot().revision_id,
        idempotency_key: Uuid::new_v4(),
        proposal,
    })?;
    println!("Committed revision: {}", result.revision_id);
    println!("{}", serde_json::to_string_pretty(engine.snapshot())?);
    Ok(())
}
