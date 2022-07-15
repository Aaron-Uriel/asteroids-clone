use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::AsciiSheet;
use std::f32;


const BASE_SIZE: f32 = 5.0;

#[derive(Inspectable, Component)]
pub struct Player {
    facing_angle: f32
}

enum Quadrant {First, Second, Third, Fourth}

#[derive(Inspectable, Component)]
pub struct Velocity {
    magnitude: f32,
    angle: f32
}

impl Velocity {
    fn new() -> Velocity {
        Velocity { magnitude: 0.0, angle: 0.0 }
    }

    fn get_quadrant(&self) -> Quadrant {
        if self.angle.cos().is_sign_positive() && self.angle.sin().is_sign_positive() {
            Quadrant::First
        }
        else if self.angle.cos().is_sign_positive() {
            Quadrant::Fourth
        }
        else if self.angle.sin().is_sign_positive() {
            Quadrant::Second
        } 
        else {
            Quadrant::Third
        }
    }

    fn get_quadrant_by_components(x_comp: f32, y_comp: f32) -> Quadrant {
        if x_comp.is_sign_positive() && y_comp.is_sign_positive() {
            Quadrant::First
        }
        else if x_comp.is_sign_positive() {
            Quadrant::Fourth
        }
        else if y_comp.is_sign_positive() {
            Quadrant::Second
        }
        else {
            Quadrant::Third
        }
    }

    fn sum(&self, vel2: Velocity) -> Velocity {
        let x_axis_sum = self.magnitude * self.angle.cos() + vel2.magnitude * vel2.angle.cos() + f32::EPSILON;
        let y_axis_sum = self.magnitude * self.angle.sin() + vel2.magnitude * vel2.angle.sin() + f32::EPSILON;

        let mag = f32::sqrt(x_axis_sum.powi(2) + y_axis_sum.powi(2));
        let angle = f32::atan(y_axis_sum / x_axis_sum) + match Velocity::get_quadrant_by_components(x_axis_sum, y_axis_sum) {
            Quadrant::First => 0.0,
            Quadrant::Second | Quadrant::Third => f32::consts::PI,
            Quadrant::Fourth => 2.0 * f32::consts::PI
        };

        Velocity { magnitude: mag, angle: angle }
    }

    fn add(&mut self, vel1: Velocity) {
        let x_axis_sum = self.magnitude * self.angle.cos() + vel1.magnitude * vel1.angle.cos() + f32::EPSILON;
        let y_axis_sum = self.magnitude * self.angle.sin() + vel1.magnitude * vel1.angle.sin() + f32::EPSILON;

        self.magnitude = f32::sqrt(x_axis_sum.powi(2) + y_axis_sum.powi(2));
        self.angle = f32::atan(y_axis_sum / x_axis_sum) + match Velocity::get_quadrant_by_components(x_axis_sum, y_axis_sum) {
            Quadrant::First => 0.0,
            Quadrant::Second | Quadrant::Third => f32::consts::PI,
            Quadrant::Fourth => 2.0 * f32::consts::PI
        };
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(modify_velocity);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(16);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(BASE_SIZE)); 

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 900.0),
        ..Default::default()
    })
    .insert(Player {facing_angle: std::f32::consts::PI / 2.0})
    .insert(Velocity::new())
    .insert(Name::new("Player"));
}

fn player_movement(
    mut query: Query<(&Player, &Velocity, &mut Transform)>
) {
    let (player, velocity, mut transform) = query.single_mut();

    transform.rotation = Quat::from_rotation_z(player.facing_angle);
    transform.translation.x += velocity.magnitude * f32::cos(velocity.angle);
    transform.translation.y += velocity.magnitude * f32::sin(velocity.angle);
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

        delta_velocity = Some(Velocity { magnitude: magnitude, angle: angle});
    }

    if keyboard.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        player.facing_angle += f32::consts::PI * time.delta_seconds() * if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
            1.0
        } else {
            -1.0
        };
    }

    if let Some(velocity_to_add) = delta_velocity {
        velocity.add(velocity_to_add);
    }
}