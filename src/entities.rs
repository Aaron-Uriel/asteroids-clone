use std::{f32, default::Default};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use rand::distributions::Uniform;
use crate::ascii_sheet::*;
use crate::consts;

#[derive(Inspectable, Component)]
pub struct Player;

#[derive(Inspectable, Component)]
pub struct Asteroid;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(create_world_border_sensor)
            .add_startup_system(spawn_player)
            .add_startup_system(spawn_asteroids)
            .add_system(world_border_system)
            .add_system(handle_player_input);
    }
}

fn setup(mut rapier_conf: ResMut<RapierConfiguration>) {
    rapier_conf.gravity = Vec2::default();
}

fn create_world_border_sensor(mut commands: Commands) {
    const BASE_HALF_HEIGHT: f32 = 150.0;
    const BASE_HALF_WIDTH: f32 = BASE_HALF_HEIGHT * crate::ASPECT_RATIO;

    commands.spawn()
        .insert(Collider::polyline(
            vec![
                Vec2::new( BASE_HALF_WIDTH,  BASE_HALF_HEIGHT),
                Vec2::new(-BASE_HALF_WIDTH,  BASE_HALF_HEIGHT),
                Vec2::new(-BASE_HALF_WIDTH, -BASE_HALF_HEIGHT),
                Vec2::new( BASE_HALF_WIDTH, -BASE_HALF_HEIGHT),
                Vec2::new( BASE_HALF_WIDTH,  BASE_HALF_HEIGHT)
            ],
            None
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor);
}

fn world_border_system(
    mut commands: Commands,
    context: Res<RapierContext>,
    mut collision_events: EventReader<CollisionEvent>,
) {/*
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, flags) = collision_event {
            if flags == CollisionEventFlags::SENSOR {
                entity1.
            }
        }
        println!("Received collision event: {:?}", collision_event);
    }*/
}

fn spawn_asteroids(mut commands: Commands, ascii: Res<AsciiSheet>) {
    use rand::prelude::*;

    let mut rng = rand::thread_rng();
    let coordinates_range: Uniform<f32> = Uniform::from(-100.0..100.0);
    let velocity_range: Uniform<f32> = Uniform::from(10.0..100.0);
    let angle_range: Uniform<f32> = Uniform::from(0.0..(2.0 * f32::consts::PI));

    for i in 0..10 {
        let abs_vel = velocity_range.sample(&mut rng);
        let angle = angle_range.sample(&mut rng);

        let new_asteroid = spawn_ascii_sprite(
            &mut commands,
            &ascii,
            0,
            Color::rgb(0.4, 0.5, 0.9),
            consts::BASE_SPRITE_SIZE
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
            .insert(Collider::cuboid(5.0, 5.0))
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

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        16,
        Color::rgb(0.3, 0.3, 0.9),
        consts::BASE_SPRITE_SIZE
    );

    commands.entity(player)
        .insert_bundle(TransformBundle::from(Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            rotation: Quat::from_rotation_z(2.0 * f32::consts::PI),
            ..Default::default()
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

fn handle_player_input(
    mut query: Query<(&mut ExternalForce, &mut Transform), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    const BASE_FORCE_MAGNITUDE: f32 = 70.0;
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