use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};

mod debug;
use bevy_rapier2d::prelude::*;
use debug::DebugPlugin;

mod entities;
use entities::EntitiesPlugin;

mod ascii_sheet;
use ascii_sheet::AsciiPlugin;

const HEIGHT: f32 = 720.0;
const ASPECT_RATIO: f32 = 4.0 / 3.0;

mod my_colors {
    use bevy::render::color::Color;

    pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(my_colors::CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * ASPECT_RATIO,
            height: HEIGHT,
            title: "Asteroids".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EntitiesPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 200.0;
    camera.projection.bottom = -200.0;

    camera.projection.right = 200.0 * ASPECT_RATIO;
    camera.projection.left = -200.0 * ASPECT_RATIO;
    
    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
