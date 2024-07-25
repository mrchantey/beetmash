use crate::prelude::*;
use beet::prelude::*;
use bevy::prelude::*;

/// In scene-based workflows apps are mainly for adding plugins and systems.
/// They should only spawn low-level entities and resources, allowing for
/// scenes to be spawned via some IO mechanism, ie fs or network.
pub fn simple_plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins.set(window_plugin()),
        PlaceholdersPlugin,
        ReplicatePlugin,
        CommonEventsPlugin,
        transport_plugin,
    ))
    .add_systems(Startup, load_scenes_from_args);
}

/// Beetmash creates a canvas for us, the id can be configured in `Cargo.toml`
fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#beetmash-canvas".into()),
            ..default()
        }),
        ..default()
    }
}

/// allow communication with the beetmash web client
fn transport_plugin(_app: &mut App) {
    #[cfg(target_arch = "wasm32")]
    _app.add_transport(WebEventClient::new_with_window());
}

/// helper for loading scenes via cli
pub fn load_scenes_from_args(_world: &mut World) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let args: Vec<String> = std::env::args().collect();
        load_scenes(_world, args);
    }
}

#[allow(unused)]
fn load_scenes(world: &mut World, args: Vec<String>) {
    // The first argument is the path to the program
    for path in args.iter().skip(1) {
        let path = format!("scenes/{}.ron", path);
        println!("Loading scene from: {path}");
        let scene = std::fs::read_to_string(path).unwrap();
        write_ron_to_world(&scene, world).unwrap();
    }
}
