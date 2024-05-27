use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct FrontYardPlugin;

#[derive(Component)]
pub struct FrontYard;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for FrontYardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_front_yard);
    }
}

fn spawn_front_yard(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: textures.grass.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(10.0, 10.0, 1.0),
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.1,
        },
    ));

    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().initial_size(Vec2::new(48.0 * 4.0, 48.0 * 2.0));
    for handle in textures.driveway.iter() {
        let id = handle.id();
        texture_atlas_builder.add_texture(Some(id), images.get(handle).unwrap());
    }
    let (_texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let handle = images.add(texture);

    commands
        .spawn((
            SpriteBundle {
                texture: handle,
                transform: Transform {
                    translation: Vec3::new(300., -240.0, 0.1),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(1.0, 10.0, 1.0),
                },
                ..Default::default()
            },
            ImageScaleMode::Tiled {
                tile_x: false,
                tile_y: true,
                stretch_value: 0.1,
            },
        ))
        .insert(FrontYard);
}
