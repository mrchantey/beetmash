use anyhow::Result;
use beetmash::prelude::TypeRegistryExporter;
use beetmash::prelude::*;

fn main() -> Result<()> {
	TypeRegistryExporter::new((
		MostDefaultPlugins,
		DefaultPlaceholderPlugin,
		UiTerminalPlugin,
	))
	.export()?;

	// SerdeTypeRegistration::

	Ok(())
}
