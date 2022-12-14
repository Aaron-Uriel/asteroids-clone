use bevy::prelude::*;
pub struct AsciiPlugin;

pub struct AsciiSheet(Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_spritesheet);
    }
}

impl AsciiSheet {
    pub fn spawn(&self, commands: &mut Commands, index: usize, color: Color) -> Entity {
        assert!(index < 256, "Index to obtain ascii sprite is out of range");
    
        let mut sprite = TextureAtlasSprite::new(index);
        sprite.color = color; 

        commands.spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: self.0.clone(),
            ..Default::default()
        })
        .id()
    }
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
        Vec2::splat(2.0),
        Vec2::splat(0.0)
    );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
