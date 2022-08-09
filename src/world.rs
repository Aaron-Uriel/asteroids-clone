use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::consts::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_world_border_sensor_system);
    }
}

fn spawn_world_border_sensor_system(mut commands: Commands) {
    const BASE_HALF_HEIGHT: f32 = 150.0;
    const BASE_HALF_WIDTH: f32 = BASE_HALF_HEIGHT * crate::ASPECT_RATIO;

    commands.spawn()
        .insert(Collider::polyline(
            vec![
                Vec2::new( BASE_HALF_WIDTH,  BASE_HALF_HEIGHT),
                Vec2::new(-BASE_HALF_WIDTH,  BASE_HALF_HEIGHT),
                Vec2::new(-BASE_HALF_WIDTH, -BASE_HALF_HEIGHT),
                Vec2::new( BASE_HALF_WIDTH, -BASE_HALF_HEIGHT),
                Vec2::new( BASE_HALF_WIDTH,  BASE_HALF_HEIGHT)
            ],
            None
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor);
}