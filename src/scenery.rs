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
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(0.5, 0.5, 1.0),
                },
                ..Default::default()
            }
        })
        .with_children(|parent| {
            // Trees
            parent.spawn((
                SpriteBundle {
                    texture: textures.tree1.clone(),
                    transform: Transform {
                        translation: Vec3::new(440.0 / 0.5, 0.0, 0.0),
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
            // Cars
            parent.spawn(
                SpriteBundle {
                    texture: textures.car_right_gray.clone(),
                    transform: Transform::from_translation(Vec3::new(340.0 / 0.5, -600.0 / 0.5, 0.0)),
                    ..Default::default()
                }
            );
            parent.spawn(
                SpriteBundle {
                    texture: textures.car_right_blue.clone(),
                    transform: Transform::from_translation(Vec3::new(340.0 / 0.5, -500.0 / 0.5, 0.0)),
                    ..Default::default()
                }
            );
            parent.spawn(
                SpriteBundle {
                    texture: textures.car_right_red.clone(),
                    transform: Transform::from_translation(Vec3::new(340.0 / 0.5, -400.0 / 0.5, 0.0)),
                    ..Default::default()
                }
            );
            // House
            parent.spawn(
                SpriteBundle {
                    texture: textures.house.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, -200.0 / 0.5, 0.0)),
                    ..Default::default()
                }
            );
        })
        .insert(Scenery);
}
