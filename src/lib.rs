#![doc = include_str!("../README.md")]
#[cfg(feature = "net")]
pub use beetmash_net as net;

pub mod prelude {
    #[cfg(feature = "net")]
    pub use beetmash_net::prelude::*;
}
