use bevy::prelude::Vec2;

pub const ASPECT_RATIO: f32 = 4.0 / 3.0;
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = HEIGHT * ASPECT_RATIO;

pub const WORLD_HALF_HEIGHT: f32 = 200.0;
pub const WORLD_HALF_WIDTH: f32 = WORLD_HALF_HEIGHT * ASPECT_RATIO;

pub const BASE_SPRITE_SIZE: Vec2 = Vec2::splat(10.0);
