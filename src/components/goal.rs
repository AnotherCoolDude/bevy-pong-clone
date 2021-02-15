use bevy::prelude::{Vec2, Vec3};
use bevy::window::WindowResized;
#[derive(Debug, Clone, Copy)]
pub enum Goal {
	Left,
	Right,
}

impl Goal {
	pub fn update_after_win_resize(&self, resize_event: &WindowResized, size: &mut Vec2, translation: &mut Vec3) {
		let win_height = resize_event.height;
		let win_width = resize_event.width;
		let (new_size, new_pos) = match self {
			Goal::Left => (Vec2::new(10., win_height + 10.), Vec3::new(-win_width * 0.5, 0., 0.)),
			Goal::Right => (Vec2::new(10., win_height + 10.), Vec3::new(win_width * 0.5, 0., 0.)),
		};

		*size = new_size;
		*translation = new_pos;
	}
}
