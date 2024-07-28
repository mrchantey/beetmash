//! This example is for exporting the example replication registry.
//! Import it into other apps for consistent reg_ids.
use crate::prelude::*;
use anyhow::Result;
use bevy::prelude::*;
use std::fs;

/// Build a replication registry and write it to a file.
/// Expects the app to have a ReplicateRegistry resource.
/// # Errors
/// If failed to write or the
pub fn build_replication_registry(world: &World) -> Result<()> {
	let registry =
		world.get_resource::<ReplicateRegistry>().ok_or_else(|| {
			anyhow::anyhow!("Failed to get ReplicateRegistry resource")
		})?;
	let json = registry.types_to_json();
	fs::create_dir_all("target").ok();
	let path = "target/replication_registry.json";
	fs::write(path, json)?;
	println!(
		"Wrote Replication registry:\nPath: {path}\nDetails:\n{}",
		registry.types_to_json()
	);
	Ok(())
}
