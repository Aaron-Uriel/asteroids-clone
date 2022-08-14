use bevy::prelude::*;
use crate::consts::*;

#[derive(Component)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(world_border_system);
    }
}

fn world_border_system(
    mut entity_query: Query<&mut Transform>
) {
    const BORDER_HALF_WIDTH: f32 = WORLD_HALF_WIDTH + WORLD_BORDER_MARGIN;
    const BORDER_HALF_HEIGHT: f32 = WORLD_HALF_HEIGHT + WORLD_BORDER_MARGIN;

    for mut entity_transform in &mut entity_query {
        if entity_transform.translation.x >=  BORDER_HALF_WIDTH
        || entity_transform.translation.x <= -BORDER_HALF_WIDTH {
            entity_transform.translation.x *= -1.0;
        }
        if entity_transform.translation.y >=  BORDER_HALF_HEIGHT
        || entity_transform.translation.y <= -BORDER_HALF_HEIGHT {
            entity_transform.translation.y *= -1.0;
        }
    }
}