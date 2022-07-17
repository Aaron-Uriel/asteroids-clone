use bevy::prelude::*;

const BASE_SIZE: f32 = 5.0;

pub struct AsciiPlugin;

pub struct AsciiSheet(Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet);
    }
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3
) -> Entity {
    assert!(index < 256, "Index to obtain ascii sprite is out of range");
    
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color; 
    sprite.custom_size = Some(Vec2::splat(BASE_SIZE)); 

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: translation,
            ..Default::default()
        },
        ..Default::default()
    })
    .id()
}

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
