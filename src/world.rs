use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{consts::*, player::{Player}, health::*};

#[derive(Component)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(world_border_system)
            .add_system(collisions_system);
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

fn collisions_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Health, &mut HealthTimer), With<Player>>,
) {
    let (player, mut player_health, mut health_timer)= player_query.single_mut();

    for collision in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision {
            if *entity1 == player || *entity2 == player {
                println!("Colisión con jugador");
                if health_timer.finished() {
                    player_health.decrease();
                    health_timer.reset();
                }
            }
        }
    }
}