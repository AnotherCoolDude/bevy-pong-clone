use crate::components::Player;
use bevy::prelude::{Vec2, Vec3};
use bevy::window::WindowResized;

pub struct Paddle {
    pub speed: f32,
    pub movement_length: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            speed: 720. / 3.,
            movement_length: 720.,
        }
    }
}

impl Paddle {
    pub const WIDTH: f32 = 20.0;
    pub const MARGIN: f32 = 50.0;

    pub fn update_after_win_resize(
        &mut self,
        resize_event: &WindowResized,
        size: &mut Vec2,
        translation: &mut Vec3,
        player: Player,
    ) {
        let win_height = resize_event.height;
        let win_width = resize_event.width;
        self.speed = resize_event.height / 3.0;
        self.movement_length = win_height;

        let new_size = Vec2::new(Paddle::WIDTH, 0.2 * win_height);
        let x_translation = match player {
            Player::Left => -win_width * 0.5 + Paddle::MARGIN,
            Player::Right => win_width * 0.5 - Paddle::MARGIN - Paddle::WIDTH,
        };
        let new_translation = Vec3::new(x_translation, 0.0, 0.0);

        *size = new_size;
        *translation = new_translation;
    }
}
