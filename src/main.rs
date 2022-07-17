use bevy::{prelude::*, render::camera::ScalingMode};

mod debug;
use debug::DebugPlugin;

mod player;
use player::PlayerPlugin;

mod math_vec;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 100.0;
    camera.orthographic_projection.bottom = -100.0;

    camera.orthographic_projection.right = 100.0 * ASPECT_RATIO;
    camera.orthographic_projection.left = -100.0 * ASPECT_RATIO;
    
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
