use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use crate::{
    ascii_sheet::*,
    consts
};

#[derive(Component, Inspectable)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system)
            .add_system(player_movement_system);
    }
}

fn spawn_player_system(mut commands: Commands, ascii: Res<AsciiSheet>) {
    use std::f32;

    let player = ascii.spawn(&mut commands, 16, Color::rgb(0.3, 0.3, 0.9));

    let player_angle = Quat::from_rotation_z(2.0 * f32::consts::PI);

    commands.entity(player)
        .insert_bundle(TransformBundle::from(Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            rotation: player_angle,
            scale: consts::BASE_SPRITE_SCALE,
        }))
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            angular_damping: 100.0,
            ..Default::default()
        })
        .insert(Collider::ball(5.0))
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(Name::new("Player"));
}

fn player_movement_system(
    mut query: Query<(&mut ExternalForce, &mut Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    const BASE_FORCE_MAGNITUDE: f32 = 100.0;
    const BASE_ROTATION: f32 = 5.0;

    let (mut ext_force, mut transform) = query.single_mut();

    if keyboard.any_pressed([KeyCode::W, KeyCode::S, KeyCode::Up, KeyCode::Down]) {
        let (axis, angle) = transform.rotation.to_axis_angle();
        let angle = if axis.z.is_sign_negative() { -angle } else { angle };

        ext_force.force = match keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
            true => Vec2::new(
                BASE_FORCE_MAGNITUDE * angle.cos() * time.delta_seconds(),
                BASE_FORCE_MAGNITUDE * angle.sin() * time.delta_seconds()
            ),
            false => Vec2::new(
                -BASE_FORCE_MAGNITUDE * angle.cos() * time.delta_seconds(),
                -BASE_FORCE_MAGNITUDE * angle.sin() * time.delta_seconds()
            )
        } 
    }
    if keyboard.any_just_released([KeyCode::W, KeyCode::S, KeyCode::Up, KeyCode::Down]) {
        ext_force.force = Vec2::default();
    }

    if keyboard.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        transform.rotate_z(match keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
            true =>   BASE_ROTATION * time.delta_seconds(),
            false => -BASE_ROTATION * time.delta_seconds()
        });
    }
}