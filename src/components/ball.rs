use bevy::prelude::{Vec2, Vec3};
use bevy::window::WindowResized;
pub struct Ball {
    pub speed: f32,
    pub direction: Vec2,
}

impl Ball {
    pub fn velocity(&self) -> Vec2 {
        self.speed * self.direction.normalize()
    }

    pub fn update_after_win_resize(
        &mut self,
        resize_event: &WindowResized,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let height = resize_event.height;
        self.speed = height / 1.5;
        let ball_width = 0.05 * height;

        *size = Vec2::new(ball_width, ball_width);
        *translation = Vec3::new(1.0, 1.0, 0.0);
    }
}

impl Ball {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            direction: Vec2::new(1.0, 1.0).normalize(),
        }
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            direction: Vec2::new(1.0, 1.0).normalize(),
        }
    }
}
