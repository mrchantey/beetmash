use beetmash_template::prelude::*;
use bevy::prelude::*;
use std::fs;

fn scene_app() -> App {
    let mut app = App::new();
    app.add_plugins(RegisterPlaceholderTypes);
    app
}

const SCENES: [(&str, fn(Commands)); 2] = [
    ("simple_scene", spawn_simple_scene),
    ("simple_environment", spawn_simple_environment),
];

fn main() {
    fs::create_dir_all("scenes").ok();

    for (name, spawn_func) in SCENES.iter() {
        // spawn scene
        let mut app = scene_app();
        let world = app.world_mut();
        spawn_func(world.commands());
        world.flush();

        // serialize scene
        let scene = DynamicScene::from_world(world);
        let str = scene
            .serialize(&world.resource::<AppTypeRegistry>().read())
            .unwrap();

        // validate scene
        scene
            .write_to_world(scene_app().world_mut(), &mut default())
            .unwrap();

        // save scene
        fs::write(format!("scenes/{name}.ron"), str).ok();
    }
}
