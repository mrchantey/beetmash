use crate::prelude::*;
use anyhow::Result;
use bevy::utils::hashbrown::HashSet;
use std::path::Path;
use ts_rs::Dependency;
use ts_rs::TypeVisitor;
use ts_rs::TS;

pub fn export_ts(path: &Path) -> Result<()> {
	SerdeTypeRegistration::export_all_to(path)?;
	let mut paths = collect::<SerdeTypeRegistration>()
		.into_iter()
		.map(|p| {
			// p.de
			format!("export * from './{}';", p.output_path.display())
		})
		.collect::<HashSet<_>>()
		.into_iter()
		.collect::<Vec<_>>();
	paths.sort();
	let paths = paths.join("\n");
	std::fs::write(path.join("index.ts"), paths)?;
	Ok(())
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
