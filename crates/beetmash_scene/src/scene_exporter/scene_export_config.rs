use crate::prelude::*;
use std::path::PathBuf;



#[derive(Debug, Clone)]
pub struct SceneExportConfig {
	/// The format to export to, currently beetmash.com only supports json.
	pub format: SceneFormat,
	pub dir: PathBuf,
	pub clear_target_dir: bool,
	pub checks: DynamicSceneChecks,
}


impl Default for SceneExportConfig {
	fn default() -> Self {
		Self {
			format: SceneFormat::Json,
			dir: PathBuf::from("scenes"),
			clear_target_dir: true,
			checks: Default::default(),
		}
	}
}
