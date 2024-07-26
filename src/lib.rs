#![doc = include_str!("../README.md")]
#[cfg(feature = "net")]
pub use beetmash_core as core;
pub use beetmash_net as net;

pub mod prelude {
    pub use crate::core::prelude::*;
    #[cfg(feature = "net")]
    pub use crate::net::prelude::*;
}
