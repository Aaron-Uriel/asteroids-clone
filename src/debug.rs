use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::{
    entities,
    physics
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<entities::Player>()
                .register_inspectable::<entities::FacingAngle>()
                .register_inspectable::<physics::MathVec32>()
                .register_inspectable::<physics::Velocity>();
        }
    }
}