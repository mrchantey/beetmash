use crate::prelude::*;
use bevy::prelude::*;
use bevy::reflect::serde::ReflectSerializer;
use bevy::reflect::TypePathTable;
use bevy::reflect::TypeRegistration;
use bevy::reflect::TypeRegistry;
use bevy_reflect::GetTypeRegistration;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// A serializable form of [TypeRegistration].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTypeRegistration {
	pub info: SerdeTypeInfo,
	pub path_table: SerdeTypePathTable,
	/// default value encoded via [`serde_json::to_string`] string, ie `{ "foo": 42 }`.
	pub default: Option<String>,
	pub docs: Option<String>,
	pub traits: SerdeTypeTraits,
}


impl SerdeTypeRegistration {
	/// Convenience method for testing
	#[allow(dead_code)]
	pub(crate) fn from_reflect<T: GetTypeRegistration>() -> Self {
		let mut registry = TypeRegistry::empty();
		registry.register::<T>();
		let registration = T::get_type_registration();
		Self::from_type_registration(&registry, &registration)
	}
	/// Convenience method for testing
	#[allow(dead_code)]
	pub(crate) fn from_reflect_with_default<
		T: Reflect + TypePath + GetTypeRegistration + Default,
	>() -> Self {
		let mut registry = TypeRegistry::empty();
		registry.register::<T>();
		registry.register_type_data::<T, ReflectDefault>();
		let registration = registry.get(std::any::TypeId::of::<T>()).unwrap();
		Self::from_type_registration(&registry, &registration)
	}



	/// We need the registry for serializing the default value, and any additional traits.
	pub fn from_type_registration(
		registry: &TypeRegistry,
		registration: &TypeRegistration,
	) -> Self {
		let type_info = registration.type_info();
		let docs = type_info.docs().map(|s| s.to_string());
		let default = map_default(registry, registration);

		Self {
			info: type_info.into(),
			path_table: type_info.type_path_table().into(),
			docs,
			default,
			traits: SerdeTypeTraits::from_registration(registration),
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
		Err(err) => {
			eprintln!(
				"Failed to serialize default value for type: {}\nError: {err}",
				reg.type_info().type_path_table().path(),
			);
			None
		}
		Ok(val) => Some(val),
	}
}


#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTypeTraits {
	component: bool,
	resource: bool,
}

impl SerdeTypeTraits {
	pub fn from_registration(reg: &TypeRegistration) -> Self {
		Self {
			component: reg.data::<ReflectComponent>().is_some(),
			resource: reg.data::<ReflectResource>().is_some(),
		}
	}
}

/// Serializable [TypePathTable].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTypePathTable {
	pub path: String,
	pub short_type_path: String,
	pub ident: Option<String>,
	pub crate_name: Option<String>,
	pub module_path: Option<String>,
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
