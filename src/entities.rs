use std::{
    f32,
    default::Default
};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii_sheet::*,
    physics::*
};

mod entity_components {
    use super::*;

    #[derive(Inspectable, Component)]
    pub struct FacingAngle(pub f32);
    
    #[derive(Inspectable, Component)]
    pub struct PhysicalAttributes {
        pub mass: Mass,
        pub vel: MathVec<Velocity>,
        pub momentum: MathVec<Momentum>
    }
    
    impl Default for PhysicalAttributes {
        fn default() -> PhysicalAttributes {
            PhysicalAttributes { mass: Mass(0.0), vel: MathVec::new(0.0, 0.0), momentum: MathVec::new(0.0, 0.0) }
        }
    }

    #[derive(Inspectable, Component)]
    pub enum UpdateEvent { Start, VelocityUpdated, MassUpdated, MomentumUpdated }
}
pub use entity_components::*;



#[derive(Inspectable, Component)]
pub struct Player;

#[derive(Inspectable, Component)]
pub struct Asteroid;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_asteroids)
            .add_system(handle_player_input)
            .add_system(physics_updater_system)
            .add_system(movement_system);
    }
}

fn spawn_asteroids(mut commands: Commands, ascii: Res<AsciiSheet>) {
    use rand::Rng;
    use std::ops::Range;

    let mut rng = rand::thread_rng();
    const COORDS_RANGE: Range<f32> = -100.0..100.0;
    const VELOCITY_RANGE: (Range<f32>, Range<f32>) = (0.0..0.3, 0.0..(2.0*f32::consts::PI));

    let mut asteroids: Vec<Entity> = Vec::new();

    for i in 0..10 {
        let new_random_coords = Vec3::new(
            rng.gen_range(COORDS_RANGE), 
            rng.gen_range(COORDS_RANGE), 
            700.0
        );
        let new_random_magnitude = rng.gen_range(VELOCITY_RANGE.0);
        let new_random_angle = rng.gen_range(VELOCITY_RANGE.1);

        let new_asteroid = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            0,
            Color::rgb(0.4, 0.5, 0.9),
            new_random_coords
        );

        commands.entity(new_asteroid)
            .insert(Asteroid)
            .insert(FacingAngle(new_random_angle))
            .insert(PhysicalAttributes {
                mass: Mass(3.0),
                vel: MathVec::new(new_random_magnitude, new_random_angle),
                ..Default::default()
            })
            .insert(UpdateEvent::Start)
            .insert(Name::new(format!("Asteroid {}", i)));
        asteroids.push(new_asteroid);
    }

    commands.spawn()
        .insert(Name::new("Asteroids"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&asteroids);
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
        .insert(Player)
        .insert(FacingAngle(std::f32::consts::PI / 2.0))
        .insert(PhysicalAttributes {
            mass: Mass(5.0),
            vel: MathVec::new(0.0, 0.0),
            ..Default::default()
        })
        .insert(UpdateEvent::Start)
        .insert(Name::new("Player"));
}

fn movement_system(
    mut query: Query<(&PhysicalAttributes, &mut Transform, &FacingAngle)>
) {
    for (physic_attrs, mut transform, facing_angle) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(facing_angle.0);
        transform.translation.x += physic_attrs.vel.magnitude * f32::cos(physic_attrs.vel.angle);
        transform.translation.y += physic_attrs.vel.magnitude * f32::sin(physic_attrs.vel.angle);
    }
}

fn handle_player_input(
    mut query: Query<(&mut PhysicalAttributes, &mut FacingAngle, &mut UpdateEvent), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (mut physic_attrs, mut facing_angle, update_event) = query.single_mut();

    let mut delta_velocity: Option<MathVec<Velocity>> = None;
    if keyboard.any_pressed([KeyCode::W, KeyCode::S, KeyCode::Up, KeyCode::Down]) {
        let magnitude: f32 =  match keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
            true =>   0.35 * time.delta_seconds(),
            false => -0.35 * time.delta_seconds()
        };
        let angle: f32 = facing_angle.0;

        delta_velocity = Some(MathVec::new(magnitude, angle));
    }

    if keyboard.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        facing_angle.0 +=  match keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
            true =>   f32::consts::PI * time.delta_seconds(),
            false => -f32::consts::PI * time.delta_seconds()
        };
    }

    if let Some(velocity_to_add) = delta_velocity {
        physic_attrs.vel.vec_add(velocity_to_add);
        *update_event.into_inner() = UpdateEvent::VelocityUpdated;
    }
}

fn physics_updater_system(mut query: Query<(&mut PhysicalAttributes, &UpdateEvent)>) {
    for (mut physical_attribs, update_event) in query.iter_mut() {
        if physical_attribs.is_changed() {
            match update_event {
                UpdateEvent::Start | UpdateEvent::VelocityUpdated | UpdateEvent::MassUpdated =>
                    physical_attribs.momentum = MathVec::new(
                        physical_attribs.mass.0 * physical_attribs.vel.magnitude, 
                        physical_attribs.vel.angle
                    ),
                
                UpdateEvent::MomentumUpdated =>
                    physical_attribs.vel = MathVec::new(
                        physical_attribs.momentum.magnitude / physical_attribs.mass.0,
                        physical_attribs.momentum.angle
                    )
            };
        }
    }
}