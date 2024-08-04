//! Test the output of the [SceneGroupExporter] and [TypeRegistryExporter]
//! Files are exported to `target/scenes` and `target/type_registry`
use anyhow::Result;
use beetmash_scene::prelude::*;
use bevy::prelude::*;
use std::borrow::Cow;

fn main() -> Result<()> {
	SceneGroupExporter::new(plugin)
		.with_checks(
			DynamicSceneChecks::default().with_num_ignored_resources(6),
		)
		.add_scene("my_scene", scene)
		.export()?;
	TypeRegistryExporter::new(plugin).export()?;

	Ok(())
}

fn scene(mut commands: Commands) {
	commands.insert_resource(MyResource((1.1, 2.2)));

	commands.spawn(Name::new("Root")).with_children(|parent| {
		parent.spawn((Name::new("Child1"), MyUnitStruct));
		parent.spawn((
			Name::new("Child2"),
			MyNewTypeStruct("hello world".into()),
		));
		parent.spawn((Name::new("Child3"), MyTupleStruct(2.2, 3.3)));
		parent
			.spawn((
				Name::new("Child4"),
				MyNamedStruct::default(),
				MyEnum::Named {
					field1: 6.6,
					field2: 7.7,
				},
			))
			.with_children(|parent| {
				parent.spawn((
					Name::new("Kitchen Sink Grandchild"),
					MyUnitStruct,
					MyNewTypeStruct::default(),
					MyTupleStruct::default(),
					MyNamedStruct::default(),
					MyEnum::default(),
				));
			})
			.with_children(|parent| {
				parent.spawn((
					Name::new("Great Grandchild"),
					MyBigStruct::default(),
				));
			});
	});
}


fn plugin(app: &mut App) {
	app
	/*-*/
	.register_type::<Name>()
	.register_type::<Parent>()
	.register_type::<Children>()
	.register_type::<MyResource>()
	.register_type::<MyUnitStruct>()
	.register_type::<MyNewTypeStruct>()
	.register_type::<MyTupleStruct>()
	.register_type::<MyNamedStruct>()
	.register_type::<MyEnum>()
	.register_type::<MyBigStruct>()
/*-*/;
}

/// A struct thats just a unit struct
#[derive(Default, Resource, Reflect)]
#[reflect(Default, Resource)]
struct MyResource((f32, f64));

/// A struct thats just a unit struct
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
struct MyUnitStruct;


/// A beautiful new type
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
struct MyNewTypeStruct(
	/// A field that is a Cow<str>
	Cow<'static, str>,
);


/// A tuple struct
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
struct MyTupleStruct(
	/// A field that is a f32
	f32,
	/// A field that is a f32
	f32,
);
/// A struct with named fields
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
struct MyNamedStruct {
	/// A field that is a f32
	field1: f32,
	/// A field that is a f32
	field2: f32,
}


/// An enum with lots of variants
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
enum MyEnum {
	/// A variant with no fields
	#[default]
	Unit,
	/// A variant with a f32 field
	NewType(
		/// NewType structs can have docs too
		f32,
	),
	/// A variant with a tuple struct
	Tuple(f32, f32),
	/// A variant with named fields
	Named {
		/// A field that is a f32
		field1: f32,
		/// A field that is a f32
		field2: f32,
	},
}



/// A struct with lots of fields
#[derive(Default, Component, Reflect)]
#[reflect(Default, Component)]
struct MyBigStruct {
	/// 8-bit signed integer
	my_i8_type: i8,
	/// 16-bit signed integer
	my_i16_type: i16,
	/// 32-bit signed integer
	my_i32_type: i32,
	/// 64-bit signed integer
	my_i64_type: i64,
	// /// 128-bit signed integer
	// my_i128_type: i128,
	/// pointer-sized signed integer
	my_isize_type: isize,
	/// 8-bit unsigned integer
	my_u8_type: u8,
	/// 16-bit unsigned integer
	my_u16_type: u16,
	/// 32-bit unsigned integer
	my_u32_type: u32,
	/// 64-bit unsigned integer
	my_u64_type: u64,
	// /// 128-bit unsigned integer
	// my_u128_type: u128,
	/// pointer-sized unsigned integer
	my_usize_type: usize,
	/// 32-bit floating-point number
	my_f32_type: f32,
	/// 64-bit floating-point number
	my_f64_type: f64,
	/// a Unicode scalar value
	my_char_type: char,
	/// a boolean value (true or false)
	my_bool_type: bool,
	my_tuple_type: (i32, f64, char),
	my_array_type: [i32; 5],
	my_vec_type: Vec<i32>,

	my_unit_struct: MyUnitStruct,
	my_newtype_struct: MyNewTypeStruct,
	my_tuple_struct: MyTupleStruct,
	my_named_struct: MyNamedStruct,
	my_enum: MyEnum,
	my_optional_field: Option<f32>,
}
