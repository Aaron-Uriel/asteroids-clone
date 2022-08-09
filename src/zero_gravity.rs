use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ZeroGravityPlugin;

impl Plugin for ZeroGravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(configure_system);
    }
}

fn configure_system(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::default();
}