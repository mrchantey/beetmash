pub mod placeholders;
pub use placeholders::*;
pub mod simple_plugin;
pub use simple_plugin::*;
pub mod simple_scene;
pub use simple_scene::*;

pub mod prelude {

    pub use crate::placeholders::*;
    pub use crate::simple_plugin::*;
    pub use crate::simple_scene::*;
}
