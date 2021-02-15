use bevy::{
	prelude::{Vec2, Vec3},
	window::WindowResized,
};

pub enum Wall {
	Top,
	Bottom,
	Left,
	Right,
}

impl Wall {
	pub fn size(&self) -> Vec2 {
		const WIDTH: f32 = 1280.0;
		const HEIGHT: f32 = 720.0;
		const WALL_WIDTH: f32 = 10.0;

		match self {
			Wall::Top | Wall::Bottom => Vec2::new(WIDTH + WALL_WIDTH, WALL_WIDTH),
			Wall::Left | Wall::Right => Vec2::new(WALL_WIDTH, HEIGHT + WALL_WIDTH),
		}
	}

	pub fn position(&self) -> Vec3 {
		const WIDTH: f32 = 1280.0;
		const HEIGHT: f32 = 720.0;

		match self {
			Wall::Top => Vec3::new(0.0, HEIGHT * 0.5, 0.0),
			Wall::Bottom => Vec3::new(0.0, -HEIGHT * 0.5, 0.0),
			Wall::Left => Vec3::new(-WIDTH * 0.5, 0.0, 0.0),
			Wall::Right => Vec3::new(WIDTH * 0.5, 0.0, 0.0),
		}
	}

	pub fn update_after_win_resize(&self, resize_event: &WindowResized, size: &mut Vec2, translation: &mut Vec3) {
		let win_height = resize_event.height;
		let win_width = resize_event.width;
		let (new_size, new_pos) = match self {
			Wall::Top => (Vec2::new(win_width + 10., 10.), Vec3::new(0., win_height * 0.5, 0.)),
			Wall::Bottom => (Vec2::new(win_width + 10., 10.), Vec3::new(0., -win_height * 0.5, 0.)),
			Wall::Left => (Vec2::new(10., win_height + 10.), Vec3::new(-win_width * 0.5, 0., 0.)),
			Wall::Right => (Vec2::new(10., win_height + 10.), Vec3::new(win_width * 0.5, 0., 0.)),
		};

		*size = new_size;
		*translation = new_pos;
	}
}
