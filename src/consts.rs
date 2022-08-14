use bevy::prelude::Vec3;

pub const ASPECT_RATIO: f32 = 4.0 / 3.0;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = HEIGHT * ASPECT_RATIO;

pub const WORLD_HALF_HEIGHT: f32 = 200.0;
pub const WORLD_HALF_WIDTH: f32 = WORLD_HALF_HEIGHT * ASPECT_RATIO;

pub const WORLD_BORDER_MARGIN: f32 = 10.0;

pub const BASE_SPRITE_SCALE: Vec3 = Vec3::new(1.2, 1.2, 1.0);

pub const ASTEROIDS_LIMIT: u16 = 10;