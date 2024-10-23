use beetmash_net::prelude::*;
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
			.add_observer(log_user_message)
			.add_observer(log_on_message.pipe(ui_terminal_stdout))
			.add_observer(log_app_ready.pipe(ui_terminal_stdout))
			.add_systems(Update, 
				parse_text_input
			)
			.add_systems(
				PostUpdate,
				(init_output,resize_output,remove_excessive_lines)
					.before(UiSystem::Layout),
			)
			.register_type::<OutputContainer>()
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
pub struct OutputItem;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Default, Component)]
pub struct OutputContainer;

fn font() -> TextFont {
	TextFont {
		font_size: 32.,
		..default()
	}
}

/// Pipe a system into here for the values to be printed to the terminal.
pub fn ui_terminal_stdout(
	text: In<String>,
	mut commands: Commands,
	terminals: Query<Entity, With<OutputContainer>>,
) {
	for entity in terminals.iter() {
		commands.entity(entity).with_children(|parent| {
			// style.color.set_alpha(0.);
			parent.spawn(
				// AccessibilityNode(NodeBuilder::new(Role::ListItem)),
				(OutputItem, Text::new(text.clone()),font()),
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

const MAX_LINES: usize = 12;
fn remove_excessive_lines(
	mut commands: Commands,
	mut list: Query<&Children, (With<OutputContainer>, Changed<Children>)>,
) {
	for children in list.iter_mut() {
		let num_over_max = children.len().saturating_sub(MAX_LINES);
		// removes the first n children
		for child in children.iter().take(num_over_max) {
			commands.entity(*child).despawn_recursive();
		}
	}
}

const INPUT_HEIGHT_PX: f32 = 50.;

fn resize_output(
	window: Single<&Window>,
	mut resize_reader: EventReader<WindowResized>,
	mut containers: Populated<&mut Node, With<OutputContainer>>,
) {
	if resize_reader.read().count() > 0 {
		for mut node in containers.iter_mut() {
			node.height = Val::Px(window.height() - INPUT_HEIGHT_PX);
		}
	}
}
fn init_output(
	window: Single<&Window>,
	mut containers: Populated<&mut Node, Added<OutputContainer>>,
) {
	for mut node in containers.iter_mut() {
		node.height = Val::Px(window.height() - INPUT_HEIGHT_PX);
	}
}

pub fn spawn_ui_terminal(mut commands: Commands, user_input: bool) {
	commands
		// ROOT CONTAINER
		.spawn(
			Node {
				width: Val::Percent(100.),
				height: Val::Percent(100.),
				justify_content: JustifyContent::SpaceBetween,
				flex_direction: FlexDirection::Column,
				..default()
			},
			// background_color: Color::srgb(0.10, 0.10, 0.10).into(),
		)
		.with_children(|root| {
			root
				.spawn((
					Name::new("Output Container"),
					OutputContainer,
					 Node {
						width: Val::Percent(100.),
						height: Val::Percent(80.), // gets overridden by init_output and resize_output
						padding: UiRect::all(Val::Px(10.)),
						flex_direction: FlexDirection::Column,
						overflow: Overflow::clip(),
						..default()
					},
				));
			if user_input {
				root
					.spawn((
						Name::new("Input Container"),
						Text::default(),
						// font(),
						// Text::new("User> "),
						// TextAlign::default(),
						Node {
							display: Display::Flex,
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							width: Val::Percent(100.),
							height: Val::Px(INPUT_HEIGHT_PX),
							padding: UiRect::all(Val::Px(10.)),
							..default()
						},
						BackgroundColor(Color::srgba(0., 0., 0., 0.2)),
					))
    				.with_child((TextSpan::new(" User> "),font()))
						.with_child((TextSpan::default(),InputContainer,font()));
			}
		});
}

fn parse_text_input(
	mut commands: Commands,
	mut evr_char: EventReader<KeyboardInput>,
	keys: Res<ButtonInput<KeyCode>>,
	mut query: Query<&mut TextSpan, With<InputContainer>>,
) {
	if keys.any_pressed([KeyCode::ControlRight, KeyCode::ControlLeft]) {
		return;
	}
	for ev in evr_char.read() {
		if let ButtonState::Released = ev.state {
			continue;
		}
		for mut text in query.iter_mut() {
			match &ev.logical_key {
				Key::Enter => {
					commands.trigger(OnUserMessage(text.0.clone()));
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
