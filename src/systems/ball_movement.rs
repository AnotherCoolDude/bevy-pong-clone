use crate::components::ball::Ball;
use bevy::prelude::*;

pub fn ball_movement(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
	let delta = time.delta_seconds();

	for (ball, mut transform) in query.iter_mut() {
		transform.translation += delta * ball.velocity().extend(0.0);
	}
}
