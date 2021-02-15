use crate::components::{Ball, Goal, Paddle, Player, Wall};
use crate::events::window_resize::*;
use bevy::prelude::*;
use bevy::window::WindowResized;

pub fn window_resize_listener(
    mut resize_listener: ResMut<WindowResizeListenerState>,
    resize_events: Res<Events<WindowResized>>,
    mut paddle_query: Query<(&Player, &mut Paddle, &mut Transform, &mut Sprite)>,
    mut ball_query: Query<(&mut Ball, &mut Transform, &mut Sprite)>,
    mut wall_query: Query<(&mut Wall, &mut Transform, &mut Sprite)>,
    mut goal_query: Query<(&mut Goal, &mut Transform, &mut Sprite)>,
) {
    if let Some(resize_event) = resize_listener.event_reader.latest(&resize_events) {
        for (player, mut paddle, mut transform, mut sprite) in paddle_query.iter_mut() {
            paddle.update_after_win_resize(
                resize_event,
                &mut sprite.size,
                &mut transform.translation,
                *player,
            );
        }

        for (mut ball, mut transform, mut sprite) in ball_query.iter_mut() {
            ball.update_after_win_resize(
                resize_event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }

        for (wall, mut transform, mut sprite) in wall_query.iter_mut() {
            wall.update_after_win_resize(
                resize_event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }

        for (goal, mut transform, mut sprite) in goal_query.iter_mut() {
            goal.update_after_win_resize(
                resize_event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }
    }
}
