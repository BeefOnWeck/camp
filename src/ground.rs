use crate::helpe::Create;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct GroundPlugin;

#[derive(Component)]
pub struct Ground;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    images: ResMut<Assets<Image>>,
) {
    // Grass
    commands.spawn(Create::tiled_bounded_sprite(
        textures.grass.clone(),
        0.0, 0.0, 0.0,
        true, true, 30.0
    ));

    // Driveway
    let driveway_texture = assemble_driveway_texture(&textures, images);
    commands.spawn(Create::tiled_bounded_sprite(
        driveway_texture.clone(),
        300.0, -240.0, 0.1,
        false, true, 10.0
    ));

    // Road
    commands.spawn((
        SpriteBundle {
            texture: driveway_texture.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -816.0, 0.1),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(20.0, 2.0, 1.0),
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 0.2,
        },
    ))
    .insert(Ground);
}

fn assemble_driveway_texture(textures: &Res<TextureAssets>, mut images: ResMut<Assets<Image>>) -> Handle<Image> {
    let mut texture_atlas_builder = 
        TextureAtlasBuilder::default().initial_size(Vec2::new(48.0 * 4.0, 48.0 * 2.0));
    for handle in textures.driveway.iter() {
        let id = handle.id();
        texture_atlas_builder.add_texture(Some(id), images.get(handle).unwrap());
    }
    let (_texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let handle = images.add(texture);
    handle
}
