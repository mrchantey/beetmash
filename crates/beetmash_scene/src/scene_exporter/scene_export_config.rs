use crate::prelude::*;
use anyhow::Result;
use bevy::scene::ron;
use serde::Serialize;
use std::path::PathBuf;

/// The format to export to, currently beetmash.com only supports json.
#[derive(Default)]
pub enum SceneExportFormat {
	#[default]
	Json,
	Ron,
}

impl SceneExportFormat {
	pub fn extension(&self) -> &'static str {
		match self {
			SceneExportFormat::Json => "json",
			SceneExportFormat::Ron => "ron",
		}
	}

	pub fn to_string<T: Serialize>(&self, value: &T) -> Result<String> {
		match self {
			SceneExportFormat::Json => Ok(serde_json::to_string_pretty(value)?),
			SceneExportFormat::Ron => {
				Ok(ron::ser::to_string_pretty(value, Default::default())?)
			}
		}
	}
}




pub struct SceneExportConfig {
	pub format: SceneExportFormat,
	pub dir: PathBuf,
	pub clear_target_dir: bool,
	pub checks: DynamicSceneChecks,
}


impl Default for SceneExportConfig {
	fn default() -> Self {
		Self {
			format: SceneExportFormat::Json,
			dir: PathBuf::from("target/scenes"),
			clear_target_dir: true,
			checks: Default::default(),
		}
	}
}
