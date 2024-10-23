//! This is published as an app to beetmash.com
use beetmash::prelude::*;
use beetmash_core::scenes;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(beetmash_full_plugins)
		.add_systems(Startup, (scenes::camera_2d, scenes::ui_terminal_input))
		.insert_resource(Tick(Timer::from_seconds(0.1, TimerMode::Repeating)))
		.add_systems(Update, populate)
		.run();
}

#[derive(Resource, Deref, DerefMut)]
struct Tick(pub Timer);


fn populate(mut timer: ResMut<Tick>, time: Res<Time>, mut commands: Commands) {
	if timer.tick(time.delta()).just_finished() {
		commands
			.trigger(OnUserMessage(format!("Tick {:.1}", time.elapsed_secs())));
	}
}
