use bevy::prelude::KeyCode;

#[derive(Clone, Copy)]
pub enum Player {
	Left,
	Right,
}

impl Player {
	pub fn keycodes(&self) -> (KeyCode, KeyCode) {
		match self {
			Player::Left => (KeyCode::W, KeyCode::S),
			Player::Right => (KeyCode::Up, KeyCode::Down),
		}
	}
}
