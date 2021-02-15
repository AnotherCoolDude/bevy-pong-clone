use crate::components::{Ball, Collider, Goal, ScoreText};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

pub fn ball_collision(
	mut ball_query: Query<(&mut Ball, &Sprite, &Transform)>,
	collider_query: Query<(&Collider, &Sprite, &Transform)>,
	mut score_text_query: Query<(&mut Text, &mut ScoreText)>,
) {
	for (mut ball, ball_sprite, ball_transform) in ball_query.iter_mut() {
		for (collider, collider_sprite, collider_transform) in collider_query.iter() {
			let collision = collide(
				ball_transform.translation,
				ball_sprite.size,
				collider_transform.translation,
				collider_sprite.size,
			);

			let collision = match collision {
				Some(collision) => collision,
				None => continue,
			};

			if let Collider::Goal(goal) = *collider {
				for (mut text, mut score) in score_text_query.iter_mut() {
					match goal {
						Goal::Left => score.score_right += 1,
						Goal::Right => score.score_left += 1,
					}
					text.value = score.to_string();
				}
			}

			use Collision::*;
			let (reflect_x, reflect_y) = match collision {
				Left => (ball.direction.x > 0.0, false),
				Right => (ball.direction.x < 0.0, false),
				Top => (false, ball.direction.y < 0.0),
				Bottom => (false, ball.direction.y > 0.0),
			};

			if reflect_x {
				ball.direction.x = -ball.direction.x;
			}

			if reflect_y {
				ball.direction.y = -ball.direction.y;
			}
		}
	}
}
