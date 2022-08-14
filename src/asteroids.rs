use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;
use crate::{
    ascii_sheet::*,
    consts
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
    let impulse_range: Uniform<f32> = Uniform::from(0.1..0.25);
    let angle_range: Uniform<f32> = Uniform::from(0.0..(2.0 * f32::consts::PI));
    let boolean_rand = Bernoulli::new(0.5).unwrap();

    let asteroids = commands.spawn()
        .insert(Name::new("Asteroids"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .insert(Visibility::visible())
        .id();

    for i in 0..consts::ASTEROIDS_LIMIT {
        let new_asteroid = ascii.spawn(&mut commands, 0, Color::rgb(0.4, 0.5, 0.9));

        let asteroid_rotation = Quat::from_rotation_z(angle_range.sample(&mut rng));

        commands.entity(new_asteroid)
            .insert_bundle(TransformBundle::from(
                Transform {
                    translation: [coordinates_range.sample(&mut rng), coordinates_range.sample(&mut rng), 700.0].into(),
                    rotation: asteroid_rotation,
                    scale: consts::BASE_SPRITE_SCALE
                }
            ))
            .insert(RigidBody::Dynamic)
            .insert(Sleeping::disabled())
            .insert(Collider::cuboid(
                2.75 * consts::BASE_SPRITE_SCALE.x,
                2.75 * consts::BASE_SPRITE_SCALE.y 
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ExternalImpulse {
                impulse: Vec2::new(
                    impulse_range.sample(&mut rng) * if boolean_rand.sample(&mut rng) { -1.0 } else { 1.0 },
                    impulse_range.sample(&mut rng) * if boolean_rand.sample(&mut rng) { -1.0 } else { 1.0 }
                ),
                ..Default::default()
            })
            .insert(Velocity::default())
            .insert(Damping {
                angular_damping: 1.0,
                ..Default::default()
            })
            .insert(Name::new(format!("Asteroid {}", i)));
        
        commands.entity(asteroids)
            .add_child(new_asteroid);
    }
    
}