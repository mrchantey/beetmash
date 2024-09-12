use crate::prelude::*;
use std::collections::HashMap;
use bevy::reflect::TypeRegistry;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// A serializable type registry
#[derive(Serialize, Deserialize, TS)]
pub struct SerdeTypeRegistry {
	/// Map of a type path, aka [std::any::type_name] to its registration.
	pub registrations: HashMap<String, SerdeTypeRegistration>,
	// pub short_path_to_type_name: HashMap<String, String>,
	// pub type_path_to_id: HashMap<&'static str, String>,
	// pub ambiguous_names: HashSet<String>,
}

impl From<&TypeRegistry> for SerdeTypeRegistry {
	fn from(registry: &TypeRegistry) -> Self {
		let registrations = registry
			.iter()
			.map(|reg| {
				(
					reg.type_info().type_path().to_string(),
					SerdeTypeRegistration::from_type_registration(
						registry, reg,
					),
				)
			})
			.collect();

		Self { registrations }
	}
}
