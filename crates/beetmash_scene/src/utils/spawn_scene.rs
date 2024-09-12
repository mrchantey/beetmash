use anyhow::Result;
use bevy::scene::ron;
use bevy::reflect::Reflect;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Reflect)]
pub enum SceneFormat {
	#[default]
	Json,
	Ron,
}

impl SceneFormat {
	pub fn extension(&self) -> &'static str {
		match self {
			SceneFormat::Json => "json",
			SceneFormat::Ron => "ron",
		}
	}

	pub fn from_path(path: &str) -> Result<Self> {
		if path.ends_with(".json") {
			return Ok(SceneFormat::Json);
		} else if path.ends_with(".ron") {
			return Ok(SceneFormat::Ron);
		}
		anyhow::bail!("Unrecognized scene extension format: {}", path)
	}

	pub fn to_string<T: Serialize>(&self, value: &T) -> Result<String> {
		match self {
			SceneFormat::Json => Ok(serde_json::to_string_pretty(value)?),
			SceneFormat::Ron => {
				Ok(ron::ser::to_string_pretty(value, Default::default())?)
			}
		}
	}
}
