use anyhow::Result;
use bevy::utils::hashbrown::HashSet;
use std::path::PathBuf;
use ts_rs::Dependency;
use ts_rs::TypeVisitor;
use ts_rs::TS;


pub struct TypescriptExporter<T> {
	dir: PathBuf,
	phantom: std::marker::PhantomData<T>,
}

impl<T: 'static + TS> TypescriptExporter<T> {
	pub fn new() -> Self {
		Self {
			dir: PathBuf::from("target/typescript"),
			phantom: std::marker::PhantomData,
		}
	}

	/// export typescript bindings to the directory
	/// This will clear the directory before exporting.
	pub fn export(&self) -> Result<()> {
		std::fs::remove_dir_all(&self.dir).ok();
		std::fs::create_dir_all(&self.dir).ok();
		T::export_all_to(&self.dir)?;
		self.export_index()?;
		Ok(())
	}

	fn export_index(&self) -> Result<()> {
		let mut paths = collect::<T>()
			.into_iter()
			.map(|p| format!("export * from './{}';", p.output_path.display()))
			.collect::<HashSet<_>>()
			.into_iter()
			.collect::<Vec<_>>();
		paths.sort();
		let paths = paths.join("\n");
		std::fs::write(self.dir.join("index.ts"), paths)?;
		Ok(())
	}
}





fn collect<T: 'static + TS>() -> Vec<Dependency> {
	let mut deps = vec![Dependency::from_ty::<T>().unwrap()];
	struct Visit<'a>(&'a mut Vec<Dependency>);
	impl<'a> TypeVisitor for Visit<'a> {
		fn visit<T2: TS + 'static + ?Sized>(&mut self) {
			if let Some(dep) = Dependency::from_ty::<T2>() {
				self.0.push(dep);
			}
			T2::visit_dependencies(&mut Visit(&mut self.0));
		}
	}
	T::visit_dependencies(&mut Visit(&mut deps));
	deps
}
