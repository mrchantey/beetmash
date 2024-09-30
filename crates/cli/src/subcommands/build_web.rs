use anyhow::Result;
use clap::ArgAction;
use forky::prelude::Subcommand;
use std::fs;
use std::process::Command;


pub struct BuildBeetmashWeb;


impl Subcommand for BuildBeetmashWeb {
	fn name(&self) -> &'static str { "build" }

	fn about(&self) -> &'static str { "Build an app for Beetmash Web" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command.arg(
			clap::Arg::new("example")
				.short('e')
				.long("example")
				.required(false)
				.action(ArgAction::Set)
				.help("Provides a config file to myprog"),
		)
	}

	// untested, i prefer justfile
	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		let example = args.get_one::<String>("example");

		println!("Building Beetmash Web app...{:?}", example);
		let crate_name = "beetmash_template";
		let app_name = "main";
		let wasm_dir = "target/wasm";

		// Build the project
		let cargo_build_status = Command::new("cargo")
			.args(&[
				"build",
				"--bin",
				"main",
				"--target",
				"wasm32-unknown-unknown",
				"--release",
			])
			.arg(app_name)
			.status()?;
		if !cargo_build_status.success() {
			anyhow::bail!("Build failed");
		}

		fs::remove_dir_all(wasm_dir).ok();
		fs::create_dir_all(wasm_dir)?;

		let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
			.unwrap_or_else(|_| "target".to_string());
		let wasm_path = format!(
			"{}/wasm32-unknown-unknown/release/{}.wasm",
			cargo_target_dir, app_name
		);
		let status = Command::new("wasm-bindgen")
			.args(&[
				"--out-name",
				"main",
				"--out-dir",
				wasm_dir,
				"--target",
				"web",
				&wasm_path,
				"--no-typescript",
			])
			.status()?;
		if !status.success() {
			anyhow::bail!("wasm-bindgen failed");
		}

		let target_dir = format!(
			"/home/pete/me/beetmash-api/target/storage/apps/{}",
			crate_name
		);
		fs::create_dir_all(&target_dir)
			.expect("Failed to create target directory");
		let status = Command::new("cp")
			.args(&["-r", &format!("{}/*", wasm_dir), &target_dir])
			.status()?;
		if !status.success() {
			anyhow::bail!("Copying files failed");
		}

		Ok(())
	}
}
