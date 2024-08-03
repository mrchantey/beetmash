use bevy::reflect::ArrayInfo;
use bevy::reflect::EnumInfo;
use bevy::reflect::ListInfo;
use bevy::reflect::MapInfo;
use bevy::reflect::NamedField;
use bevy::reflect::StructInfo;
use bevy::reflect::StructVariantInfo;
use bevy::reflect::TupleInfo;
use bevy::reflect::TupleStructInfo;
use bevy::reflect::TupleVariantInfo;
use bevy::reflect::TypeInfo;
use bevy::reflect::UnitVariantInfo;
use bevy::reflect::UnnamedField;
use bevy::reflect::ValueInfo;
use bevy::reflect::VariantInfo;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "kind")]
pub enum SerdeTypeInfo {
	#[serde(rename = "struct")]
	Struct(SerdeStructInfo),
	#[serde(rename = "tupleStruct")]
	TupleStruct(SerdeTupleStructInfo),
	#[serde(rename = "tuple")]
	Tuple(SerdeTupleInfo),
	#[serde(rename = "list")]
	List(SerdeListInfo),
	#[serde(rename = "array")]
	Array(SerdeArrayInfo),
	#[serde(rename = "map")]
	Map(SerdeMapInfo),
	#[serde(rename = "enum")]
	Enum(SerdeEnumInfo),
	#[serde(rename = "value")]
	Value(SerdeValueInfo),
}

impl From<&TypeInfo> for SerdeTypeInfo {
	fn from(info: &TypeInfo) -> Self {
		match info {
			TypeInfo::Struct(info) => Self::Struct(info.into()),
			TypeInfo::TupleStruct(info) => Self::TupleStruct(info.into()),
			TypeInfo::Tuple(info) => Self::Tuple(info.into()),
			TypeInfo::List(info) => Self::List(info.into()),
			TypeInfo::Array(info) => Self::Array(info.into()),
			TypeInfo::Map(info) => Self::Map(info.into()),
			TypeInfo::Enum(info) => Self::Enum(info.into()),
			TypeInfo::Value(info) => Self::Value(info.into()),
		}
	}
}

/// Serializable [NamedField].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeNamedField {
	name: String,
	type_path: String,
	docs: Option<String>,
}

impl From<&NamedField> for SerdeNamedField {
	fn from(field: &NamedField) -> Self {
		Self {
			name: field.name().to_string(),
			type_path: field.type_path().to_string(),
			docs: field.docs().map(|s| s.to_string()),
		}
	}
}

/// Serializable [UnnamedField].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeUnnamedField {
	index: usize,
	type_path: String,
	docs: Option<String>,
}

impl From<&UnnamedField> for SerdeUnnamedField {
	fn from(field: &UnnamedField) -> Self {
		Self {
			index: field.index(),
			type_path: field.type_path().to_string(),
			docs: field.docs().map(|s| s.to_string()),
		}
	}
}


/// Serializable [StructInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeStructInfo {
	fields: Vec<SerdeNamedField>,
}

impl From<&StructInfo> for SerdeStructInfo {
	fn from(info: &StructInfo) -> Self {
		Self {
			fields: info.iter().map(SerdeNamedField::from).collect(),
		}
	}
}
/// Serializable [TupleStructInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTupleStructInfo {
	fields: Vec<SerdeUnnamedField>,
}

impl From<&TupleStructInfo> for SerdeTupleStructInfo {
	fn from(info: &TupleStructInfo) -> Self {
		Self {
			fields: info.iter().map(SerdeUnnamedField::from).collect(),
		}
	}
}
/// Serializable [TupleInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTupleInfo {
	fields: Vec<SerdeUnnamedField>,
}

impl From<&TupleInfo> for SerdeTupleInfo {
	fn from(info: &TupleInfo) -> Self {
		Self {
			fields: info.iter().map(SerdeUnnamedField::from).collect(),
		}
	}
}

/// Serializable [ListInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeListInfo {
	item_type_path: String,
}

impl From<&ListInfo> for SerdeListInfo {
	fn from(info: &ListInfo) -> Self {
		Self {
			item_type_path: info.item_type_path_table().path().to_string(),
		}
	}
}

/// Serializable [ArrayInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeArrayInfo {
	item_type_path: String,
	capacity: usize,
}

impl From<&ArrayInfo> for SerdeArrayInfo {
	fn from(info: &ArrayInfo) -> Self {
		Self {
			item_type_path: info.item_type_path_table().path().to_string(),
			capacity: info.capacity(),
		}
	}
}
/// Serializable [MapInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeMapInfo {
	key_type_path: String,
	value_type_path: String,
}

impl From<&MapInfo> for SerdeMapInfo {
	fn from(info: &MapInfo) -> Self {
		Self {
			key_type_path: info.key_type_path_table().path().to_string(),
			value_type_path: info.value_type_path_table().path().to_string(),
		}
	}
}

/// Serializable [ValueInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeValueInfo {
	item_type_path: String,
}

impl From<&ValueInfo> for SerdeValueInfo {
	fn from(info: &ValueInfo) -> Self {
		Self {
			item_type_path: info.type_path().to_string(),
		}
	}
}


/// Serializable [EnumInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeEnumInfo {
	variants: Vec<SerdeVariantInfo>,
}

impl From<&EnumInfo> for SerdeEnumInfo {
	fn from(info: &EnumInfo) -> Self {
		Self {
			variants: info.iter().map(SerdeVariantInfo::from).collect(),
		}
	}
}
/// Serializable [VariantInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "kind")]
pub enum SerdeVariantInfo {
	#[serde(rename = "struct")]
	Struct(SerdeStructVariantInfo),
	#[serde(rename = "tuple")]
	Tuple(SerdeTupleVariantInfo),
	#[serde(rename = "unit")]
	Unit(SerdeUnitVariantInfo),
}

impl From<&VariantInfo> for SerdeVariantInfo {
	fn from(info: &VariantInfo) -> Self {
		match info {
			VariantInfo::Struct(info) => Self::Struct(info.into()),
			VariantInfo::Tuple(info) => Self::Tuple(info.into()),
			VariantInfo::Unit(info) => Self::Unit(info.into()),
		}
	}
}

/// Serializable [StructVariantInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeStructVariantInfo {
	name: String,
	fields: Vec<SerdeNamedField>,
	docs: Option<String>,
}

impl From<&StructVariantInfo> for SerdeStructVariantInfo {
	fn from(info: &StructVariantInfo) -> Self {
		Self {
			name: info.name().to_string(),
			fields: info.iter().map(SerdeNamedField::from).collect(),
			docs: info.docs().map(|s| s.to_string()),
		}
	}
}
/// Serializable [TupleVariantInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeTupleVariantInfo {
	name: String,
	fields: Vec<SerdeUnnamedField>,
	docs: Option<String>,
}

impl From<&TupleVariantInfo> for SerdeTupleVariantInfo {
	fn from(info: &TupleVariantInfo) -> Self {
		Self {
			name: info.name().to_string(),
			fields: info.iter().map(SerdeUnnamedField::from).collect(),
			docs: info.docs().map(|s| s.to_string()),
		}
	}
}
/// Serializable [UnitVariantInfo].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct SerdeUnitVariantInfo {
	name: String,
	docs: Option<String>,
}

impl From<&UnitVariantInfo> for SerdeUnitVariantInfo {
	fn from(info: &UnitVariantInfo) -> Self {
		Self {
			name: info.name().to_string(),
			docs: info.docs().map(|s| s.to_string()),
		}
	}
}
