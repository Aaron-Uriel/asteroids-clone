use bevy::{prelude::*, render::camera::ScalingMode};

mod debug;
use debug::DebugPlugin;

mod player;
use player::PlayerPlugin;

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
        .add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet)
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

struct AsciiSheet(Handle<TextureAtlas>);

fn load_spritesheet(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0)
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
