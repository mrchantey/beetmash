use anyhow::Result;
use beetmash::prelude::TypeRegistryExporter;
use beetmash::prelude::*;

fn main() -> Result<()> {
	TypeRegistryExporter::new((
		MostDefaultPlugins,
		DefaultPlaceholderPlugin,
		UiTerminalPlugin,
	))
	.with_name("all_types")
	.export()?;

	// SerdeTypeRegistration::

	Ok(())
}
