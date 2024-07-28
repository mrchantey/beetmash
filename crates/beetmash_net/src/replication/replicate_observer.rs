use crate::prelude::*;
use anyhow::Result;
use bevy::prelude::*;
use forky_core::ResultTEExt;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Functions for handling reception of [`Event`] triggers.
#[derive(Copy, Clone)]
pub struct ObserverFns {
	pub send: fn(&mut Commands, payload: &MessagePayload) -> Result<()>,
}

impl ObserverFns {
	pub fn new<T: Event + DeserializeOwned>() -> Self {
		Self {
			send: |commands, payload| {
				commands.trigger(payload.deserialize::<T>()?);
				Ok(())
			},
		}
	}
}

pub fn register_observer_outgoing<T: Event + Serialize>(app: &mut App) {
	app.world_mut().observe(outgoing_send::<T>);
}

fn outgoing_send<T: Event + Serialize>(
	trigger: Trigger<T>,
	registrations: Res<ReplicateRegistry>,
	mut outgoing: ResMut<MessageOutgoing>,
) {
	let Some(payload) =
		MessagePayload::new(trigger.event()).ok_or(|e| log::error!("{e}"))
	else {
		return;
	};
	outgoing.push(
		Message::SendObserver {
			reg_id: registrations.registration_id::<T>(),
			payload,
		}
		.into(),
	);
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use beetmash_core::prelude::*;
	use bevy::prelude::*;
	use serde::Deserialize;
	use serde::Serialize;
	use sweet::*;

	#[derive(Debug, Clone, Event, Serialize, Deserialize, PartialEq)]
	pub struct MyEvent(pub i32);

	#[test]
	fn outgoing() -> Result<()> {
		let mut app = App::new();
		app.add_plugins(ReplicatePlugin)
			.replicate_observer_outgoing::<MyEvent>();

		app.world_mut().flush_trigger(MyEvent(7));

		app.update();

		let msg_out = app.world_mut().resource_mut::<MessageOutgoing>();
		expect(msg_out.len()).to_be(1)?;
		expect(&msg_out[0]).to_be(
			&Message::SendObserver {
				reg_id: RegistrationId::new_with(0),
				payload: MessagePayload::new(&MyEvent(7))?,
			}
			.into(),
		)?;

		Ok(())
	}

	#[test]
	fn incoming() -> Result<()> {
		let mut app1 = App::new();
		app1.add_plugins(ReplicatePlugin)
			.replicate_observer_outgoing::<MyEvent>();
		let mut app2 = App::new();

		app2.add_plugins(ReplicatePlugin)
			.replicate_observer_incoming::<MyEvent>();


		// Send
		app1.world_mut().flush_trigger(MyEvent(7));
		app1.update();
		Message::loopback(app1.world_mut(), app2.world_mut());


		let msg_in = app2.world_mut().resource_mut::<MessageIncoming>();
		expect(msg_in.len()).to_be(1)?;

		let on_trigger = observe_triggers::<MyEvent>(app2.world_mut());

		app2.update();

		expect(&on_trigger).to_have_been_called_times(1)?;
		expect(&on_trigger).to_have_returned_nth_with(0, &MyEvent(7))?;

		Ok(())
	}
}
