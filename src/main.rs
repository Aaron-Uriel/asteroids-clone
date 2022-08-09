use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};

mod debug;
use bevy_rapier2d::prelude::*;
use debug::DebugPlugin;

mod world;
use world::WorldPlugin;

mod player;
use player::PlayerPlugin;

mod asteroids;
use asteroids::AsteroidsPlugin;

mod zero_gravity;
use zero_gravity::ZeroGravityPlugin;

mod ascii_sheet;
use ascii_sheet::AsciiPlugin;

mod consts;
use consts::*;

mod my_colors {
    use bevy::render::color::Color;

    pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(my_colors::CLEAR))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
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
        .add_plugin(PlayerPlugin)
        .add_plugin(AsteroidsPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(ZeroGravityPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = WORLD_HALF_HEIGHT;
    camera.projection.bottom = -WORLD_HALF_HEIGHT;

    camera.projection.right = WORLD_HALF_WIDTH;
    camera.projection.left = -WORLD_HALF_WIDTH;
    
    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
