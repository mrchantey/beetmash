use crate::prelude::*;
use bevy::prelude::ReflectDefault;
use bevy::reflect::TypePathTable;
use bevy::reflect::TypeRegistration;
use bevy_reflect::serde::ReflectSerializer;
use bevy_reflect::TypeRegistry;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// A serializable form of [TypeRegistration].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTypeRegistration {
	info: SerdeTypeInfo,
	path_table: SerdeTypePathTable,
	default: Option<String>,
	docs: Option<String>,
}


impl SerdeTypeRegistration {
	/// We need the registry for serializing the default value.
	pub fn from_type_registration(
		registry: &TypeRegistry,
		reg: &TypeRegistration,
	) -> Self {
		let type_info = reg.type_info();
		let docs = type_info.docs().map(|s| s.to_string());
		let default = map_default(registry, reg);

		Self {
			info: type_info.into(),
			path_table: type_info.type_path_table().into(),
			docs,
			default,
		}
	}
}

fn map_default(
	registry: &TypeRegistry,
	reg: &TypeRegistration,
) -> Option<String> {
	let Some(reflect_default) = reg.data::<ReflectDefault>() else {
		return None;
	};
	let default = reflect_default.default();
	let reflect_serializer =
		ReflectSerializer::new(default.as_ref(), &registry);

	match serde_json::to_string(&reflect_serializer) {
		Err(_err) => {
			// eprintln!(
			// 	"Failed to serialize default value for type: {:?}\nError: {:?}",
			// 	reg.type_info().type_path_table().path(),
			// 	err
			// );
			None
		}
		Ok(val) => Some(val),
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTypePathTable {
	path: String,
	short_type_path: String,
	ident: Option<String>,
	crate_name: Option<String>,
	module_path: Option<String>,
}

impl From<&TypePathTable> for SerdeTypePathTable {
	fn from(table: &TypePathTable) -> Self {
		Self {
			path: table.path().to_string(),
			short_type_path: table.short_path().to_string(),
			ident: table.ident().map(|s| s.to_string()),
			crate_name: table.crate_name().map(|s| s.to_string()),
			module_path: table.module_path().map(|s| s.to_string()),
		}
	}
}
