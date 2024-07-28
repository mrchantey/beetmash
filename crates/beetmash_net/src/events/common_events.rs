use crate::prelude::*;
// use beet_ecs::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/**
 * This adds common replication events to the app.
 * It should be added before any other events in order
 * to preserve the registration ids:
 *
 * - `0`: AppReady
 * - `1`: SpawnScene
*/
pub struct CommonEventsPlugin;

impl Plugin for CommonEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            // AppStartup
            .add_event::<AppStartup>()
            .replicate_event_outgoing::<AppStartup>()
            .add_systems(Startup, |mut events: EventWriter<AppStartup>| {
                events.send(AppStartup);
            })
            // AppReady
            .add_event::<AppReady>()
            .replicate_observer_outgoing::<AppReady>()
            .replicate_observer_incoming::<OnUserMessage>()
            .replicate_observer_outgoing::<OnAppMessage>();
        // Screenshot
    }
}

/// Sent from this app on the Startup schedule.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Event, Reflect)]
#[reflect(Default)]
pub struct AppStartup;
/// Sent from this app, usually once assets are ready.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Event, Reflect)]
#[reflect(Default)]
pub struct AppReady;

/// User messages received either internally or externally, can be treated like an StdIn.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize, Event, Reflect)]
pub struct OnUserMessage(pub String);
/// App messages for outputting, can be treated like an StdOut.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize, Event, Reflect)]
pub struct OnAppMessage(pub String);
