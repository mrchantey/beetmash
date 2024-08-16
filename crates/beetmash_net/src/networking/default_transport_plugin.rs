#[allow(unused)]
use crate::prelude::*;
use bevy::prelude::*;


pub struct DefaultTransportPlugin;
impl Plugin for DefaultTransportPlugin {
	#[allow(unused)]
	fn build(&self, app: &mut App) {
		#[cfg(target_arch = "wasm32")]
		app.add_transport(WebEventClient::new_with_window());

		#[cfg(feature = "tokio")]
		app.add_plugins(NativeClientPlugin::default());
	}
}
