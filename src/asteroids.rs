use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use crate::{
    ascii_sheet::*,
    consts::*
};

#[derive(Component, Inspectable)]
pub struct Asteroid;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_asteroids_system);
    }
}

fn spawn_asteroids_system(mut commands: Commands, ascii: Res<AsciiSheet>) {
    use std::f32;
    use rand::prelude::*;
    use rand::distributions::Uniform;

    let mut rng = rand::thread_rng();
    let coordinates_range: Uniform<f32> = Uniform::from(-100.0..100.0);
    let velocity_range: Uniform<f32> = Uniform::from(10.0..100.0);
    let angle_range: Uniform<f32> = Uniform::from(0.0..(2.0 * f32::consts::PI));

    for i in 0..ASTEROIDS_LIMIT {
        let abs_vel = velocity_range.sample(&mut rng);
        let angle = angle_range.sample(&mut rng);

        let new_asteroid = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            0,
            Color::rgb(0.4, 0.5, 0.9),
            BASE_SPRITE_SIZE
        );

        commands.entity(new_asteroid)
            .insert_bundle(TransformBundle::from(Transform {
                translation: [coordinates_range.sample(&mut rng), coordinates_range.sample(&mut rng), 700.0].into(),
                rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            }))
            .insert(Asteroid)
            .insert(RigidBody::Dynamic)
            .insert(Damping {
                angular_damping: 100.0,
                ..Default::default()
            })
            .insert(Collider::cuboid(
                BASE_SPRITE_SIZE.x / 2.0, 
                BASE_SPRITE_SIZE.y / 2.0
            ))
            .insert(Velocity {
                linvel: Vec2::new(
                    abs_vel * angle.cos(),
                    abs_vel * angle.sin()
                ),
                ..Default::default()
            })
            .insert(Name::new(format!("Asteroid {}", i)));
    }
}