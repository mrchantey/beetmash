use beetmash_net::prelude::*;
use beetmash_scene::utils::BundlePlaceholder;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::ui::UiSystem;
use bevy::window::WindowResized;
use std::borrow::Cow;

#[derive(Clone)]
pub struct UiTerminalPlugin;

impl Plugin for UiTerminalPlugin {
	fn build(&self, app: &mut App) {
		app
		.observe(log_user_message)
			.observe(log_on_message.pipe(ui_terminal_stdout))
			.observe(log_app_ready.pipe(ui_terminal_stdout))
			.add_systems(Update, 
				parse_text_input
			)
			.add_systems(
				PostUpdate,
				(init_output,resize_output_container,remove_excessive_lines)
					.before(UiSystem::Layout),
			)
			.add_systems(
				PostUpdate,
				(scroll_to_bottom_on_resize, scroll_to_bottom_on_append,show_new_sections)
					.after(UiSystem::Layout),
			)
			.register_type::<UiTerminal>()
			.register_type::<InputContainer>()
			.register_type::<OutputContainer>()
			.register_type::<OutputItem>()
			/*-*/;
	}
}

/// An 'stdout observer', triggering this will log to the ui terminal.
#[derive(Debug, Event, Deref)]
pub struct OnLogMessage(pub Cow<'static, str>);

impl OnLogMessage {
	pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
		Self(message.into())
	}
}


#[derive(Debug, Default, Component, Reflect)]
#[reflect(Default, Component)]
pub struct InputContainer;


#[derive(Debug, Default, Component, Reflect)]
#[reflect(Default, Component)]
pub struct UiTerminal;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Default, Component)]
pub struct OutputItem;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Default, Component)]
pub struct OutputContainer;

fn style() -> TextStyle {
	TextStyle {
		font_size: 32.,
		..Default::default()
	}
}

/// Pipe a system into here for the values to be printed to the terminal.
pub fn ui_terminal_stdout(
	text: In<String>,
	mut commands: Commands,
	terminals: Query<Entity, With<UiTerminal>>,
) {
	for entity in terminals.iter() {
		commands.entity(entity).with_children(|parent| {
			let mut style = style();
			style.color.set_alpha(0.);
			parent.spawn(
				// AccessibilityNode(NodeBuilder::new(Role::ListItem)),
				(OutputItem, TextBundle::from_section(text.clone(), style)),
			);
		});
	}
}

fn log_on_message(trigger: Trigger<OnLogMessage>)->String {
	trigger.event().0.to_string()
}

fn log_app_ready(_trigger: Trigger<AppReady>) -> String {
	format!("Event: AppReady")
}

fn get_top_pos(node: &Node, parent: &Node) -> f32 {
	let items_height = node.size().y;
	let container_height = parent.size().y;
	let max_scroll = (items_height - container_height).max(0.);
	// log::info!("\nitems_height: {items_height}\ncontainer_height: {container_height}\nmax_scroll: {max_scroll}");
	return -max_scroll;
}

fn scroll_to_bottom_on_resize(
	mut resize_reader: EventReader<WindowResized>,
	containers_added: Query<(), Added<UiTerminal>>,
	parents: Query<&Node>,
	mut list: Query<(&mut Style, &Node, &Parent), With<UiTerminal>>,
) {
	let should_update =
		resize_reader.read().count() > 0 || containers_added.iter().count() > 0;
	if should_update {
		for (mut style, node, parent) in list.iter_mut() {
			if let Ok(parent) = parents.get(**parent) {
				style.top = Val::Px(get_top_pos(node, parent));
			}
		}
	}
}

fn scroll_to_bottom_on_append(
	mut list: Query<
		(&mut Style, &Node, &Parent),
		(With<UiTerminal>, Changed<Children>),
	>,
	parents: Query<&Node>,
) {
	for (mut style, node, parent) in list.iter_mut() {
		if let Ok(parent) = parents.get(**parent) {
			style.top = Val::Px(get_top_pos(node, parent));
		}
	}
}

const MAX_LINES: usize = 32;
fn remove_excessive_lines(
	mut commands: Commands,
	mut list: Query<&Children, (With<UiTerminal>, Changed<Children>)>,
) {
	for children in list.iter_mut() {
		let num_over_max = children.len().saturating_sub(MAX_LINES);
		for child in children.iter().take(num_over_max) {
			commands.entity(*child).despawn_recursive();
		}
	}
}
fn show_new_sections(mut query: Query<&mut Text, Added<OutputItem>>) {
	for mut text in query.iter_mut() {
		text.sections[0].style.color.set_alpha(1.);
	}
}

const INPUT_HEIGHT_PX: f32 = 50.;

fn resize_output_container(
	mut resize_reader: EventReader<WindowResized>,
	window: Query<&Window>,
	containers_added: Query<(), Added<OutputContainer>>,
	mut containers: Query<&mut Style, With<OutputContainer>>,
) {
	let should_update =
		resize_reader.read().count() > 0 || containers_added.iter().count() > 0;
	if should_update {
		let Ok(window) = window.get_single() else {
			return;
		};
		for mut style in containers.iter_mut() {
			style.height = Val::Px(window.height() - INPUT_HEIGHT_PX);
		}
	}
}
fn init_output(
	window: Query<&Window>,
	mut containers: Query<&mut Style, Added<OutputContainer>>,
) {
	for window in window.iter() {
		for mut style in containers.iter_mut() {
			style.height = Val::Px(window.height() - INPUT_HEIGHT_PX);
		}
	}
}

pub fn spawn_ui_terminal(mut commands: Commands, user_input: bool) {
	commands
		// ROOT CONTAINER
		.spawn(NodeBundle {
			style: Style {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				justify_content: JustifyContent::SpaceBetween,
				flex_direction: FlexDirection::Column,
				..default()
			},
			// background_color: Color::srgb(0.10, 0.10, 0.10).into(),
			..default()
		})
		.with_children(|parent| {
			// OUTPUT_CONTAINER
			parent
				.spawn((OutputContainer, NodeBundle {
					style: Style {
						// flex_grow: 1.,
						width: Val::Percent(100.),
						height: Val::Percent(80.), // gets overridden by init_output and resize_output
						flex_direction: FlexDirection::Column,
						overflow: Overflow::clip(),
						..default()
					},
					// background_color: Color::srgb(0.10, 0.10, 0.10).into(),
					..default()
				}))
				.with_children(|parent| {
					parent
						// LIST
						.spawn((
							UiTerminal,
							NodeBundle {
								style: Style {
									padding: UiRect::all(Val::Px(10.)),
									flex_direction: FlexDirection::Column,
									..default()
								},
								..default()
							},
							// AccessibilityNode(NodeBuilder::new(Role::List)),
						));
					// ))
					// .with_children(|parent| {
					// 	// SCROLL TEST ITEMS
					// 	for i in 0..30 {
					// 		parent.spawn(
					// 			// AccessibilityNode(NodeBuilder::new(Role::ListItem)),
					// 			TextBundle::from_section(
					// 				format!("Item {i}"),
					// 				style(),
					// 			),
					// 		);
					// 	}
					// });
				});
			// INPUT_CONTAINER
			if user_input {
				parent
					.spawn(NodeBundle {
						style: Style {
							width: Val::Percent(100.),
							height: Val::Px(INPUT_HEIGHT_PX),
							padding: UiRect::all(Val::Px(10.)),
							..default()
						},
						background_color: Color::srgba(0., 0., 0., 0.2).into(),
						..default()
					})
					.with_children(|input_area| {
						input_area.spawn((
							BundlePlaceholder::Text(vec![
								TextSection::new("User> ", style()),
								TextSection::new("", style()),
							]),
							InputContainer,
						));
					});
			}
		});
}

fn parse_text_input(
	mut commands: Commands,
	mut evr_char: EventReader<KeyboardInput>,
	keys: Res<ButtonInput<KeyCode>>,
	mut query: Query<&mut Text, With<InputContainer>>,
) {
	if keys.any_pressed([KeyCode::ControlRight, KeyCode::ControlLeft]) {
		return;
	}
	for ev in evr_char.read() {
		if let ButtonState::Released = ev.state {
			continue;
		}
		for mut text in query.iter_mut() {
			let text = &mut text.sections[1].value; // first index is ' > '
			match &ev.logical_key {
				Key::Enter => {
					commands.trigger(OnUserMessage(text.clone()));
					text.clear();
				}
				Key::Backspace => {
					text.pop();
				}
				Key::Space => {
					text.push(' ');
				}
				Key::Character(char) => {
					text.push_str(char);
				}
				_ => {}
			}
		}
	}
}

fn log_user_message(trigger: Trigger<OnUserMessage>, mut commands: Commands) {
	commands.trigger(OnLogMessage::new(format!("User: {}", &trigger.event().0)))
}
