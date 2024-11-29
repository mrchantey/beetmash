//! json schema generated via claude
//! from https://github.com/DefinitelyTyped/DefinitelyTyped/blob/a7b4fad09a7a1309d0fa9a1ec22c57720bf5d02a/types/json-schema/index.d.ts
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
pub struct JsonSchemaBuilder;

impl JsonSchemaBuilder {
	pub fn reference(name: &str) -> JsonSchema7 {
		JsonSchema7 {
			reference: Some(name.to_string()),
			..default()
		}
	}
	pub fn object(fields: &Vec<SerdeNamedField>) -> JsonSchema7 {
		let mut properties = HashMap::new();
		for field in fields {
			properties.insert(
				field.name.clone(),
				JsonSchemaBuilder::reference(&field.type_path).into(),
			);
		}


		JsonSchema7 {
			properties: Some(properties),
			..default()
		}
	}

	pub fn tuple(fields: &Vec<SerdeUnnamedField>) -> JsonSchema7 {
		let mut items = Vec::new();
		for field in fields {
			items.push(JsonSchemaBuilder::reference(&field.type_path).into());
		}
		JsonSchema7 {
			items: Some(items.into()),
			..default()
		}
	}

	pub fn array_unbounded(item: &str) -> JsonSchema7 {
		let ty: JsonSchema7Definition =
			JsonSchemaBuilder::reference(item).into();
		JsonSchema7 {
			items: Some(ty.into()),
			..default()
		}
	}

	pub fn array_bounded(item: &str, min: u64, max: u64) -> JsonSchema7 {
		let ty: JsonSchema7Definition =
			JsonSchemaBuilder::reference(item).into();
		JsonSchema7 {
			items: Some(ty.into()),
			min_items: Some(min),
			max_items: Some(max),
			..default()
		}
	}
}


/// Primitive type names for JSON Schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JsonSchema7TypeName {
	String,
	Number,
	Integer,
	Boolean,
	Object,
	Array,
	Null,
}

/// Represents a JSON Schema v7 type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchema7Type {
	String(String),
	Number(f64),
	Boolean(bool),
	Object(HashMap<String, JsonSchema7Type>),
	Array(Vec<JsonSchema7Type>),
	Null,
}

/// JSON Schema v7 definition (can be a schema or a boolean)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchema7Definition {
	Schema(Box<JsonSchema7>),
	Boolean(bool),
}

impl Into<JsonSchema7Definition> for JsonSchema7 {
	fn into(self) -> JsonSchema7Definition {
		JsonSchema7Definition::Schema(Box::new(self))
	}
}
impl Into<JsonSchema7Definition> for bool {
	fn into(self) -> JsonSchema7Definition {
		JsonSchema7Definition::Boolean(self)
	}
}

/// Full JSON Schema v7 structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonSchema7 {
	/// Unique identifier for the schema
	#[serde(rename = "$id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,

	/// Reference to another schema
	#[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
	pub reference: Option<String>,

	/// JSON Schema version
	#[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
	pub schema_version: Option<String>,

	/// Schema comment
	#[serde(rename = "$comment", skip_serializing_if = "Option::is_none")]
	pub comment: Option<String>,

	/// Definitions dictionary
	#[serde(rename = "$defs", skip_serializing_if = "Option::is_none")]
	pub defs: Option<HashMap<String, JsonSchema7Definition>>,

	/// Type or types of the schema
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<JsonSchemaOneOrMany<JsonSchema7TypeName>>,

	/// Enumeration of allowed values
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#enum: Option<Vec<JsonSchema7Type>>,

	/// Constant value
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#const: Option<JsonSchema7Type>,

	// Numeric constraints
	#[serde(skip_serializing_if = "Option::is_none")]
	pub multiple_of: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub maximum: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclusive_maximum: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub exclusive_minimum: Option<f64>,

	// String constraints
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_length: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_length: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pattern: Option<String>,

	// Array constraints
	#[serde(skip_serializing_if = "Option::is_none")]
	pub items: Option<JsonSchemaOneOrMany<JsonSchema7Definition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_items: Option<JsonSchema7Definition>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_items: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_items: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unique_items: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub contains: Option<JsonSchema7Definition>,

	// Object constraints
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_properties: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_properties: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub properties: Option<HashMap<String, JsonSchema7Definition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pattern_properties: Option<HashMap<String, JsonSchema7Definition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_properties: Option<JsonSchema7Definition>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dependencies: Option<HashMap<String, JsonSchemaDependencyDefinition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub property_names: Option<JsonSchema7Definition>,

	// Conditional subschemas
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#if: Option<JsonSchema7Definition>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub then: Option<JsonSchema7Definition>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#else: Option<JsonSchema7Definition>,

	// Combining schemas
	#[serde(skip_serializing_if = "Option::is_none")]
	pub all_of: Option<Vec<JsonSchema7Definition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub any_of: Option<Vec<JsonSchema7Definition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub one_of: Option<Vec<JsonSchema7Definition>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub not: Option<JsonSchema7Definition>,

	// Metadata
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub default: Option<JsonSchema7Type>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub read_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub write_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub examples: Option<JsonSchema7Type>,

	// Content handling
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_media_type: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_encoding: Option<String>,

	// Legacy definitions (for backwards compatibility)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub definitions: Option<HashMap<String, JsonSchema7Definition>>,
}

/// Helper enum to support either a single item or multiple items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaOneOrMany<T> {
	Single(T),
	Multiple(Vec<T>),
}

impl<T> From<T> for JsonSchemaOneOrMany<T> {
	fn from(item: T) -> Self { JsonSchemaOneOrMany::Single(item) }
}

impl<T> From<Vec<T>> for JsonSchemaOneOrMany<T> {
	fn from(items: Vec<T>) -> Self { JsonSchemaOneOrMany::Multiple(items) }
}


/// Dependency can be either a schema or a list of required properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaDependencyDefinition {
	Schema(JsonSchema7Definition),
	Properties(Vec<String>),
}
