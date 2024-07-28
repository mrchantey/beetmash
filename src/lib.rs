#![doc = include_str!("../README.md")]
pub use beetmash_core as core;
#[cfg(feature = "net")]
pub use beetmash_net as net;
#[cfg(feature = "scene")]
pub use beetmash_scene as scene;

pub mod prelude {
	pub use crate::core::prelude::*;
	#[cfg(feature = "net")]
	pub use crate::net::prelude::*;
	#[cfg(feature = "scene")]
	pub use crate::scene::prelude::*;
}
