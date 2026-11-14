use living_source_core::{Proposal, Snapshot};
use schemars::schema_for;
use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    fs::create_dir_all(&directory)?;
    let mut proposal = serde_json::to_value(schema_for!(Proposal))?;
    proposal["properties"]["protocol_version"]["const"] = serde_json::json!(1);
    proposal["properties"]["operations"]["minItems"] = serde_json::json!(1);
    proposal["properties"]["operations"]["maxItems"] = serde_json::json!(100);
    for (name, value) in [
        ("proposal.schema.json", proposal),
        (
            "snapshot.schema.json",
            serde_json::to_value(schema_for!(Snapshot))?,
        ),
    ] {
        fs::write(
            directory.join(name),
            format!("{}\n", serde_json::to_string_pretty(&value)?),
        )?;
    }
    Ok(())
}
