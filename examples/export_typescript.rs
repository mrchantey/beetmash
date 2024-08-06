use anyhow::Result;
use beetmash::prelude::*;


fn main() -> Result<()> {
	TypescriptExporter::<SerdeTypeRegistry>::new().export()?;
	Ok(())
}
