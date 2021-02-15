use bevy::prelude::*;
use bevy::window::WindowResized;

#[derive(Default)]
pub struct WindowResizeListenerState {
	pub event_reader: EventReader<WindowResized>,
}
