use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct SceneryPlugin;

#[derive(Component)]
pub struct Scenery;

impl Plugin for SceneryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_scenery);
    }
}

fn spawn_scenery(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn({
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(440.0, 0.0, 1.0),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(0.5, 0.5, 1.0),
                },
                ..Default::default()
            }
        })
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: textures.tree1.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(1.0, 15.0, 1.0),
                    },
                    ..Default::default()
                },
                ImageScaleMode::Tiled {
                    tile_x: false,
                    tile_y: true,
                    stretch_value: 1.0 / 15.0,
                },
            ));
        })
        .insert(Scenery);
}
