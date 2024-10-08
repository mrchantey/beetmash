use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::view::screenshot::save_to_disk;
use bevy::render::view::screenshot::Screenshot;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Event, Reflect)]
pub struct SaveScreenshot {
	pub filename: String,
}


pub fn screenshot_on_event(
	trigger: Trigger<SaveScreenshot>,
	mut commands: Commands,
) {
	let filename = trigger.event().filename.clone();
	log::info!("Saved screenshot to {}", filename);
	commands
		.spawn(Screenshot::primary_window())
		.observe(save_to_disk(filename));
}

/// Take a screenshot when ctrl+s is pressed
pub fn screenshot_on_keypress(
	mut commands: Commands,
	// _trigger: Trigger<KeyboardInput>,
	mut events: EventReader<KeyboardInput>,
	keys: Res<ButtonInput<KeyCode>>,
	mut counter: Local<u32>,
) {
	if events.read().count() == 0 {
		return;
	}
	if keys.any_pressed([KeyCode::ControlRight, KeyCode::ControlLeft])
		&& keys.just_pressed(KeyCode::KeyS)
	{
		#[cfg(not(target_arch = "wasm32"))]
		std::fs::create_dir_all("target/screenshots").ok();
		#[cfg(not(target_arch = "wasm32"))]
		let path = format!("target/screenshots/screenshot-{}.png", *counter);
		#[cfg(target_arch = "wasm32")]
		let path = format!("screenshot-{}.png", *counter);
		*counter += 1;
		log::info!("Saved screenshot to {}", path);
		commands
			.spawn(Screenshot::primary_window())
			.observe(save_to_disk(path));
	}
}
