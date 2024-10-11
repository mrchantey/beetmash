use anyhow::Result;
use beetmash::prelude::*;

fn main() -> Result<()> {
	TypeRegistryExporter::new((
		MostDefaultPlugins,
		DefaultPlaceholderPlugin,
		UiTerminalPlugin,
	))
	.export()?;

	ReplicateRegistryExporter::new(DefaultReplicatePlugin).export()?;

	// SerdeTypeRegistration::

	Ok(())
}
