use crate::prelude::*;

pub trait IntoJsonSchema {
	fn into_json_schema(&self) -> JsonSchema7;
}

impl IntoJsonSchema for SerdeTypeRegistration {
	fn into_json_schema(&self) -> JsonSchema7 {
		let mut schema = self.info.into_json_schema();
		schema.title = Some(self.path_table.path.clone());
		schema.description = self.docs.clone();
		schema.default = self.default.as_ref().map(|val| {
			serde_json::from_str(val)
				.expect("failed to deserialize default value")
		});

		schema
	}
}


impl IntoJsonSchema for SerdeTypeInfo {
	fn into_json_schema(&self) -> super::JsonSchema7 {
		match self {
			SerdeTypeInfo::Struct(info) => info.into_json_schema(),
			SerdeTypeInfo::TupleStruct(info) => info.into_json_schema(),
			SerdeTypeInfo::Tuple(info) => info.into_json_schema(),
			SerdeTypeInfo::List(info) => info.into_json_schema(),
			SerdeTypeInfo::Array(info) => info.into_json_schema(),
			SerdeTypeInfo::Map(info) => info.into_json_schema(),
			SerdeTypeInfo::Set(info) => info.into_json_schema(),
			SerdeTypeInfo::Enum(info) => info.into_json_schema(),
			SerdeTypeInfo::Opaque(info) => info.into_json_schema(),
		}
	}
}

impl IntoJsonSchema for SerdeStructInfo {
	fn into_json_schema(&self) -> JsonSchema7 {
		JsonSchemaBuilder::object(&self.fields)
	}
}

impl IntoJsonSchema for SerdeTupleStructInfo {
	fn into_json_schema(&self) -> JsonSchema7 {
		JsonSchemaBuilder::tuple(&self.fields)
	}
}

impl IntoJsonSchema for SerdeTupleInfo {
	fn into_json_schema(&self) -> JsonSchema7 {
		JsonSchemaBuilder::tuple(&self.fields)
	}
}

impl IntoJsonSchema for SerdeListInfo {
	fn into_json_schema(&self) -> JsonSchema7 {
		JsonSchemaBuilder::array_unbounded(&self.item_type_path)
	}
}

impl IntoJsonSchema for SerdeArrayInfo {
	fn into_json_schema(&self) -> JsonSchema7 {
		JsonSchemaBuilder::array_bounded(
			&self.item_type_path,
			self.capacity as u64,
			self.capacity as u64,
		)
	}
}

impl IntoJsonSchema for SerdeMapInfo {
	fn into_json_schema(&self) -> JsonSchema7 { todo!() }
}

impl IntoJsonSchema for SerdeSetInfo {
	fn into_json_schema(&self) -> JsonSchema7 { todo!() }
}

impl IntoJsonSchema for SerdeEnumInfo {
	fn into_json_schema(&self) -> JsonSchema7 { todo!() }
}

impl IntoJsonSchema for SerdeOpaqueInfo {
	fn into_json_schema(&self) -> JsonSchema7 {
		JsonSchemaBuilder::reference(&self.item_type_path)
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use bevy::prelude::*;

	#[derive(Default, Reflect)]
	pub struct MyStruct {
		foo: f32,
		bar: String,
	}

	#[test]
	fn works() -> Result<()> {
		let info =
			SerdeTypeRegistration::from_reflect_with_default::<MyStruct>();

		let schema = info.into_json_schema();
		let json = serde_json::to_string_pretty(&schema)?;

		println!("{json}");

		Ok(())
	}
}
