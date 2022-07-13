use bevy::prelude::*;

const HEIGHT: f32 = 480.0;
const ASPECT_RATIO: f32 = 4.0 / 3.0;

mod MyColors {
    use bevy::render::color::Color;

    pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(MyColors::CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * ASPECT_RATIO,
            height: HEIGHT,
            title: "Asteroids".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}