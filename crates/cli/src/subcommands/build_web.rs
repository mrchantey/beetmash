use anyhow::Result;
use clap::ArgAction;
use forky::prelude::Subcommand;
use std::fs;
use std::path::PathBuf;
use std::process::Command;


pub struct BuildBeetmashWeb;


impl Subcommand for BuildBeetmashWeb {
	fn name(&self) -> &'static str { "build" }

	fn about(&self) -> &'static str {
		r#"
Build an app for Beetmash Web.
This tool is similar to trunk but with a focus on binaries and scenes
instead of html and other web assets.
"#
	}

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command
			.arg(
				clap::Arg::new("example")
					.short('e')
					.long("example")
					.required(false)
					.action(ArgAction::Set)
					.help("Specify the example name"),
			)
			.arg(
				clap::Arg::new("package")
					.short('p')
					.long("package")
					.required(false)
					.action(ArgAction::Set)
					.help("Specify the crate name"),
			)
			.arg(
				clap::Arg::new("skip-build")
					.long("skip-build")
					.required(false)
					.action(ArgAction::SetTrue)
					.help("Skip cargo build, wasm-bindgen and wasm-opt"),
			)
			.arg(
				clap::Arg::new("release")
					.long("release")
					.required(false)
					.action(ArgAction::SetTrue)
					.help("Build for release and optimize"),
			)
			.arg(
				clap::Arg::new("out-dir")
					.short('o')
					.long("out-dir")
					.default_value("target/wasm")
					.action(ArgAction::Set)
					.help("Build for release and optimize"),
			)
			.arg(
				clap::Arg::new("copy-local")
					.long("copy-local")
					.action(ArgAction::Set)
					.help("Copy wasm files to a local directory"),
			)
			.arg(
				clap::Arg::new("copy-scenes")
					.long("copy-scenes")
					.action(ArgAction::Set)
					.help("Copy scenes to directory specified by copy-local"),
			)
			.arg(
				clap::Arg::new("copy-registries")
					.long("copy-registries")
					.action(ArgAction::Set)
					.help(
						"Copy registries to directory specified by copy-local",
					),
			)
		// .arg(
		// 	clap::Arg::new("commit-local")
		// 		.long("commit-local")
		// 		.required(false)
		// 		.action(ArgAction::SetTrue)
		// 		.help("Commit all and push in directory specified by copy-local"),
		// )
	}

	// untested, i prefer justfile
	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		let args = Args::from_args(args);

		println!("Building Beetmash Web app...\n{:#?}", args);
		
		run_cargo_build(&args)?;
		run_wasm_bindgen(&args)?;
		run_wasm_opt(&args)?;
		run_copy_local(&args)?;
		// run_commit_local(&args)?;

		println!("Build Succeeded");
		Ok(())
	}
}

#[derive(Debug, Clone)]
struct Args {
	crate_name: Option<String>,
	example: Option<String>,
	app_name: String,
	release: bool,
	out_dir: String,
	copy_local: Option<String>,
	copy_scenes: Option<String>,
	copy_registries: Option<String>,
	// commit_local: bool,
	skip_build: bool,
}

impl Args {
	fn from_args(args: &clap::ArgMatches) -> Self {
		let example = args.get_one::<String>("example").cloned();
		let crate_name = args.get_one::<String>("package").cloned();
		let out_dir = args.get_one::<String>("out-dir").unwrap().clone();
		let release = args.get_flag("release");
		let skip_build = args.get_flag("skip-build");
		let app_name = example.clone().unwrap_or_else(|| "main".into());
		let copy_local = args.get_one::<String>("copy-local").cloned();
		let copy_scenes = args.get_one::<String>("copy-scenes").cloned();
		let copy_registries =
			args.get_one::<String>("copy-registries").cloned();
		// let commit_local = args.get_flag("commit-local");

		Self {
			example,
			crate_name,
			out_dir,
			skip_build,
			release,
			app_name,
			copy_local,
			copy_scenes,
			copy_registries,
			// commit_local,
		}
	}
	fn cargo_build_wasm_path(&self) -> String {
		let build_config = if self.release { "release" } else { "debug" };

		let cargo_target_dir = std::env::var("CARGO_TARGET_DIR")
			.unwrap_or_else(|_| "target".to_string());

		let mut path = PathBuf::from(cargo_target_dir);
		path.push("wasm32-unknown-unknown");
		path.push(build_config);
		if let Some(example) = &self.example {
			path.push(format!("examples/{}.wasm", example));
		} else {
			path.push(format!("{}.wasm", self.app_name));
		}
		path.to_string_lossy().to_string()
	}

	fn wasm_bindgen_path_wasm(&self) -> String {
		format!("{}/{}_bg.wasm", self.out_dir, self.app_name)
	}
	fn wasm_bindgen_path_js(&self) -> String {
		format!("{}/{}.js", self.out_dir, self.app_name)
	}
}

fn run_cargo_build(args: &Args) -> Result<()> {
	if args.skip_build {
		return Ok(());
	}

	let mut build_args = vec!["build", "--target", "wasm32-unknown-unknown"];
	if let Some(crate_name) = &args.crate_name {
		build_args.push("-p");
		build_args.push(crate_name);
	}
	if args.release {
		build_args.push("--release");
	}
	if let Some(example) = &args.example {
		build_args.push("--example");
		build_args.push(example);
	}

	// Build the project
	let status = Command::new("cargo").args(&build_args).status()?;
	if !status.success() {
		anyhow::bail!("cargo build failed");
	}
	run_print_size("Size - cargo build", &args.cargo_build_wasm_path())?;

	Ok(())
}


fn run_wasm_bindgen(args: &Args) -> Result<()> {
	if args.skip_build {
		return Ok(());
	}

	fs::create_dir_all(&args.out_dir).ok();
	let wasm_path = args.cargo_build_wasm_path();

	let build_args = [
		"--out-name",
		&args.app_name,
		"--out-dir",
		&args.out_dir,
		"--target",
		"web",
		"--no-typescript",
		&wasm_path,
	];
	// println!("wasm-bindgen {}", build_args.join(" "));

	let status = Command::new("wasm-bindgen").args(&build_args).status()?;
	if !status.success() {
		anyhow::bail!("wasm-bindgen failed");
	}

	run_print_size("Size - wasm-bindgen", &args.wasm_bindgen_path_wasm())?;

	Ok(())
}

fn run_wasm_opt(args: &Args) -> Result<()> {
	if args.skip_build || !args.release {
		return Ok(());
	}

	let wasm_bindgen_path = args.wasm_bindgen_path_wasm();

	let status = Command::new("wasm-opt")
		.args(&["-Oz", "--output", &wasm_bindgen_path, &wasm_bindgen_path])
		.status()?;
	if !status.success() {
		anyhow::bail!("wasm-opt failed");
	}

	run_print_size("Size - wasm-opt", &args.wasm_bindgen_path_wasm())?;

	Ok(())
}

fn run_print_size(prefix: &str, path: &str) -> Result<()> {
	let metadata = fs::metadata(path)?;
	let size_b = metadata.len();
	let size_mb = size_b as f64 / 1024.0 / 1024.0;
	println!("{prefix}: {:.0} MB", size_mb);
	Ok(())
}


fn run_copy_local(args: &Args) -> Result<()> {
	let Some(target_dir) = &args.copy_local else {
		return Ok(());
	};

	let crate_name = args.crate_name.clone().unwrap_or_else(|| {
		std::env::current_dir()
			.unwrap()
			.file_name()
			.unwrap()
			.to_string_lossy()
			.to_string()
	});
	let target_dir = PathBuf::from(target_dir).canonicalize()?.join(crate_name);
	fs::create_dir_all(&target_dir).ok();
	fs::copy(
		&args.wasm_bindgen_path_wasm(),
		target_dir.join(format!("{}_bg.wasm", args.app_name)),
	)?;
	fs::copy(
		&args.wasm_bindgen_path_js(),
		target_dir.join(format!("{}.js", args.app_name)),
	)?;

	if let Some(scenes_dir_src) = &args.copy_scenes {
		forky::fs::utility::fs::copy_recursive(
			scenes_dir_src,
			target_dir.join("scenes"),
		)?;
	}
	if let Some(registries_dir_src) = &args.copy_registries {
		forky::fs::utility::fs::copy_recursive(
			registries_dir_src,
			target_dir.join("registries"),
		)?;
	}

	Ok(())
}

// fn run_commit_local(args: &Args) -> Result<()> {
// 	if !args.commit_local {
// 		return Ok(());
// 	}

// 	let Some(target_dir) = &args.copy_local else {
// 		anyhow::bail!("copy-local is required for commit-local");
// 	};

// 	let target_dir = PathBuf::from(target_dir).canonicalize()?;
// 	let target_dir = target_dir.to_string_lossy();
// 	let target_dir_cmd = format!("cd {}", target_dir);

// 	let commands = vec![
// 		&target_dir_cmd,
// 		"&& git config --global user.name \"github-actions[bot]\"",
// 		"&& git config --global user.email \"github-actions[bot]@users.noreply.github.com\"",
// 		"&& git add .",
// 		"&& git commit -m \"Deploy from GitHub Actions\"",
// 		"&& git push origin main",
// 	];

// 	println!("Running commands: {:#?}", target_dir);

// 	let status = parse_commands(commands).status()?;
// 	if !status.success() {
// 		anyhow::bail!("commit failed");
// 	}

// 	Ok(())
// }


// fn parse_commands(commands: Vec<&str>) -> Command {
// 	let mut command = Command::new(&commands[0]);
// 	for c in commands.iter().skip(1) {
// 		let split_cmd = c.split_whitespace().collect::<Vec<&str>>();
// 		command.args(split_cmd);
// 	}
// 	command
// }
