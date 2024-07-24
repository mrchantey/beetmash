use beet::prelude::*;
use beetmash_template::prelude::*;
use bevy::prelude::*;

/// In scene-based workflows apps should only spawn
/// low-level entities and resources, allowing for
/// scenes to be sent via network request etc
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlaceholdersPlugin,
            ReplicatePlugin,
            CommonEventsPlugin,
            Transport,
        ))
        .add_systems(Startup, load_scenes_from_args)
        .run();
}

struct Transport;
impl Plugin for Transport {
    fn build(&self, _app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        _app.add_transport(WebEventClient::new_with_window());
    }
}

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
