use crate::prelude::*;
// use beet_ecs::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/**
 * This adds common replication events to the app.
 * It should be added before any other replications are registered in order
 * to preserve the registration ids
 *
*/
pub struct CommonEventsPlugin;

impl Plugin for CommonEventsPlugin {
	fn build(&self, app: &mut App) {
		app.replicate_observer_outgoing::<AppStartup>()
			.add_systems(Startup, app_startup)
			.replicate_observer_outgoing::<AppReady>()
			.replicate_observer_incoming::<OnUserMessage>()
			.replicate_observer_outgoing::<OnAppMessage>();
	}
}

fn app_startup(mut commands: Commands) { commands.trigger(AppStartup); }

/// Sent from this app on the Startup schedule.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Event, Reflect)]
#[reflect(Default)]
pub struct AppStartup;
/// Sent from this app, usually once assets are ready.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Event, Reflect)]
#[reflect(Default)]
pub struct AppReady;

/// User messages received either internally or externally, can be treated like an StdIn.
#[derive(
	Debug,
	Default,
	Clone,
	Deref,
	DerefMut,
	Serialize,
	Deserialize,
	Event,
	Reflect,
)]
pub struct OnUserMessage(pub String);

impl OnUserMessage {
	pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
}


/// App messages for outputting, can be treated like an StdOut.
#[derive(
	Debug,
	Default,
	Clone,
	Deref,
	DerefMut,
	Serialize,
	Deserialize,
	Event,
	Reflect,
)]
pub struct OnAppMessage(pub String);
