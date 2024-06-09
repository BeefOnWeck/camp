use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::scenery::{Bounding, Scenery};
use crate::GameState;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                animate_sprite_system.run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn((
            SpriteBundle {
                texture: textures.sprite_walk.clone(),
                transform: Transform::from_translation(Vec3::new(250.0, -600.0, 2.0)),
                ..Default::default()
            },
            TextureAtlas::from(textures.sprite_layout.clone()),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn(Camera2dBundle {
                camera: Camera {
                    order: 2,
                    ..default()
                },
                ..default()
            });
        })
        .insert(Player);
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
    actions: Res<Actions>,
) {
    for (mut timer, mut sprite) in &mut sprites_to_animate {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            match actions.player_movement {
                Some(movement) => {
                    if movement.y < 0. {
                        sprite.index = (sprite.index + 1) % 6;
                    } else if movement.y > 0. {
                        sprite.index = 12 + ((sprite.index + 1) % 6);
                    } else if movement.x > 0. {
                        sprite.index = 6 + ((sprite.index + 1) % 6);
                    } else if movement.x < 0. {
                        sprite.index = 18 + ((sprite.index + 1) % 6);
                    } else {
                        sprite.index = (sprite.index + 1) % 6;
                    }
                }
                None => {
                    sprite.index = sprite.index;
                }
            }
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    images: Res<Assets<Image>>,
    mut player_query: Query<(&mut Transform, &Handle<Image>), With<Player>>,
    bounding_query: Query<&Bounding, (With<Parent>, Without<Player>)>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.0,
    );
    for (mut player_transform, _player_image) in &mut player_query {
        let mut undo_movement = false;
        player_transform.translation += movement;
        // TODO: Figure out how to get the correct size from the sprite sheet
        let player_dimensions = Vec2::new(32.0, 47.0);
        let mut player_bounds = Aabb2d::new(
            player_transform.translation.truncate(),
            player_dimensions / 2.0,
        );
        player_bounds = player_bounds.shrink(Vec2::new(10.0, 10.0));
        info!("Player BB: {:?}", player_bounds);
        for bounding in &bounding_query {
            // TODO: Generalize to vector
            let scenery_bounds = bounding.boxes[0];
            info!("Scenery BB: {:?}", scenery_bounds);
            if scenery_bounds.intersects(&player_bounds) {
                undo_movement = true;
            }
        }
        if undo_movement == true {
            player_transform.translation -= movement;
        }
    }
}
