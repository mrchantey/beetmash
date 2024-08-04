use anyhow::Result;





pub fn ron_to_json(ron_str: &str) -> Result<String> {
	let ron_val = ron::de::from_str::<ron::Value>(ron_str)?;
	let json_str = serde_json::to_string_pretty(&ron_val)?;
	Ok(json_str)
}

pub fn json_to_ron(json_str: &str) -> Result<String> {
	let json_val = serde_json::from_str::<serde_json::Value>(json_str)?;
	let ron_str = ron::ser::to_string_pretty(&json_val, Default::default())?;
	Ok(ron_str)
}


#[cfg(test)]
mod test {
	use anyhow::Result;
	use serde::Deserialize;
	use serde::Serialize;
	use sweet::*;

	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
	struct MyStruct {
		name: String,
		age: u32,
	}

	#[test]
	#[ignore = "ron cant convert from json https://github.com/ron-rs/ron/issues/543"]
	fn ron_to_json() -> Result<()> {
		let val = MyStruct {
			name: "John".to_string(),
			age: 32,
		};
		let ron_str = ron::ser::to_string_pretty(&val, Default::default())?;
		let json = crate::utils::ron_to_json(&ron_str)?;
		let ron_str2 = crate::utils::json_to_ron(&json)?;
		let val2 = ron::de::from_str::<MyStruct>(&ron_str2)?;
		expect(val).to_be(val2)?;

		println!("{}", ron_str);
		println!("{}", json);
		println!("{}", ron_str2);


		Ok(())
	}
}
