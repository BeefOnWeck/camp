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

fn spawn_front_yard(mut commands: Commands, textures: Res<TextureAssets>) {
    let transform = Transform {
        translation: Vec3::new(0.,0.,0.),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(10.0, 10.0, 0.0),
    };
    commands
        .spawn((
            SpriteBundle {
                texture: textures.grass.clone(),
                transform,
                ..Default::default()
            },
            ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 0.1
            }
        ))
        .insert(FrontYard);
}