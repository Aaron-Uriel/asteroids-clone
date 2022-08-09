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
    use rand::distributions::{Uniform, Bernoulli};

    let mut rng = rand::thread_rng();
    let coordinates_range: Uniform<f32> = Uniform::from(-100.0..100.0);
    let impulse_range: Uniform<f32> = Uniform::from(0.25..0.5);
    let boolean_rand = Bernoulli::new(0.5).unwrap();

    for i in 0..ASTEROIDS_LIMIT {
        let impulse = Vec2::new(
            impulse_range.sample(&mut rng) * if boolean_rand.sample(&mut rng) { -1.0 } else { 1.0 },
            impulse_range.sample(&mut rng) * if boolean_rand.sample(&mut rng) { -1.0 } else { 1.0 }
        );

        let new_asteroid = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            0,
            Color::rgb(0.4, 0.5, 0.9),
            BASE_SPRITE_SIZE
        );
        commands.entity(new_asteroid)
            .insert_bundle(TransformBundle::from(Transform::from_xyz(
                coordinates_range.sample(&mut rng),
                coordinates_range.sample(&mut rng),
                coordinates_range.sample(&mut rng)
            )))
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
            .insert(ExternalImpulse {
                impulse: impulse,
                ..Default::default()
            })
            .insert(Name::new(format!("Asteroid {}", i)));
    }
}