use std::f32;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii_sheet::*,
    math_vec::MathVec32
};

#[derive(Inspectable, Component)]
pub struct Player {
    facing_angle: f32
}


#[derive(Inspectable, Component)]
pub struct Velocity(MathVec32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(modify_velocity);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        16,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(0.0, 0.0, 900.0)
    );

    commands.entity(player)
        .insert(Player {facing_angle: std::f32::consts::PI / 2.0})
        .insert(Velocity(MathVec32::new(0.0, 0.0)))
        .insert(Name::new("Player"));
}

fn player_movement(
    mut query: Query<(&Player, &Velocity, &mut Transform)>
) {
    let (player, velocity, mut transform) = query.single_mut();

    transform.rotation = Quat::from_rotation_z(player.facing_angle);
    transform.translation.x += velocity.0.get_magnitude() * f32::cos(velocity.0.get_angle());
    transform.translation.y += velocity.0.get_magnitude() * f32::sin(velocity.0.get_angle());
}

fn modify_velocity(
    mut query: Query<(&mut Player, &mut Velocity)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut player, mut velocity) = query.single_mut();

    let mut delta_velocity: Option<Velocity> = None;
    if keyboard.any_pressed([KeyCode::W, KeyCode::S, KeyCode::Up, KeyCode::Down]) {
        let magnitude: f32 = 0.5 * time.delta_seconds() * if keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
            1.0
        } else {
            -1.0
        };
        let angle: f32 = player.facing_angle;

        delta_velocity = Some(Velocity(MathVec32::new(magnitude, angle)));
    }

    if keyboard.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        player.facing_angle += f32::consts::PI * time.delta_seconds() * if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
            1.0
        } else {
            -1.0
        };
    }

    if let Some(velocity_to_add) = delta_velocity {
        velocity.0.vec_add(velocity_to_add.0);
    }
}