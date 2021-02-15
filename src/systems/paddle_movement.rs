use crate::components::{paddle::Paddle, player::Player};
use bevy::prelude::*;

pub fn paddle_movement(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Player, &Paddle, &mut Transform, &Sprite)>,
) {
	for (player, paddle, mut transform, sprite) in query.iter_mut() {
		let mut direction: f32 = 0.0;

		if keyboard_input.pressed(player.keycodes().0) {
			direction += 1.0;
		}

		if keyboard_input.pressed(player.keycodes().1) {
			direction -= 1.0;
		}

		let translation = &mut transform.translation;
		translation.y += time.delta_seconds() * direction * paddle.speed;
		translation.y = translation
			.y
			.min(paddle.movement_length * 0.5 - sprite.size.y * 0.5)
			.max(-paddle.movement_length * 0.5 + sprite.size.y * 0.5);
	}
}
