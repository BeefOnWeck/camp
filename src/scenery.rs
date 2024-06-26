use crate::helpe::{Bounding,Create};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

use crate::player::Player;

pub struct SceneryPlugin;

#[derive(Component)]
pub struct Scenery;

impl Plugin for SceneryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_scenery, compute_bounding_boxes.after(spawn_scenery)),
        );
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
            parent.spawn(Create::tiled_bounded_sprite(
                textures.tree1.clone(),
                440.0 / 0.5,
                0.0,
                0.0,
                false,
                true,
                15.0,
            ));
            // Cars
            parent.spawn(Create::bounded_sprite(
                textures.car_right_gray.clone(),
                340.0 / 0.5,
                -600.0 / 0.5,
                0.0,
            ));
            parent.spawn(Create::bounded_sprite(
                textures.car_right_blue.clone(),
                340.0 / 0.5,
                -500.0 / 0.5,
                0.0,
            ));
            parent.spawn(Create::bounded_sprite(
                textures.car_right_red.clone(),
                340.0 / 0.5,
                -400.0 / 0.5,
                0.0,
            ));
            // House
            parent.spawn(Create::bounded_sprite(
                textures.house.clone(),
                0.0,
                -200.0 / 0.5,
                0.0,
            ));
            // Fence horizontal
            parent.spawn((
                SpriteBundle {
                    texture: textures.fence_horizontal.clone(),
                    transform: Transform {
                        translation: Vec3::new(-280.0 / 0.5, -300.0 / 0.5, -0.1),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(15.0, 2.0, 1.0),
                    },
                    ..Default::default()
                },
                ImageScaleMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0 / 15.0,
                },
                Bounding::new(),
            ));
            // Fence corner
            parent.spawn((
                SpriteBundle {
                    texture: textures.fence_left_corner.clone(),
                    transform: Transform {
                        translation: Vec3::new(-472.0 / 0.5, -300.0 / 0.5, -0.1),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(1.0, 2.0, 1.0),
                    },
                    ..Default::default()
                },
                Bounding::new(),
            ));
            // Fence vertical
            parent.spawn(Create::tiled_bounded_sprite(
                textures.fence_vertical.clone(),
                -483.0 / 0.5,
                0.0 / 0.5,
                -0.2,
                false,
                true,
                60.0,
            ));
            // Back fence
            parent.spawn((
                SpriteBundle {
                    texture: textures.fence_horizontal.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0 / 0.5, 730.0 / 0.5, -0.1),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(40.0, 2.0, 1.0),
                    },
                    ..Default::default()
                },
                ImageScaleMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0 / 40.0,
                },
                Bounding::new(),
            ));
            // Front fence (invisible and just a barrier)
            parent.spawn((
                SpriteBundle {
                    texture: textures.fence_horizontal.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0 / 0.5, -730.0 / 0.5, -2.0),
                        rotation: Quat::IDENTITY,
                        scale: Vec3::new(40.0, 2.0, 1.0),
                    },
                    ..Default::default()
                },
                ImageScaleMode::Tiled {
                    tile_x: true,
                    tile_y: false,
                    stretch_value: 1.0 / 40.0,
                },
                Bounding::new(),
            ));
        })
        .insert(Scenery);
}

fn compute_bounding_boxes(
    images: Res<Assets<Image>>,
    scenery_parent: Query<&Transform, (With<Scenery>, Without<Player>)>,
    mut scenery_child: Query<
        (&Parent, &Transform, &Handle<Image>, &mut Bounding),
        (With<Parent>, Without<Player>),
    >,
) {
    for (parent, child_transform, image_handle, mut bounding) in &mut scenery_child {
        let parent_transform = scenery_parent.get(parent.get()).unwrap();
        let location = parent_transform.translation.truncate()
            + parent_transform.scale.truncate() * child_transform.translation.truncate();
        let image_size = images.get(image_handle).unwrap().size();
        let mut scaled_image_dimensions = Vec2::new(image_size.x as f32, image_size.y as f32);
        scaled_image_dimensions *= parent_transform.scale.truncate();
        scaled_image_dimensions *= child_transform.scale.truncate();
        let scenery_bounds = Aabb2d::new(location, scaled_image_dimensions / 2.0);
        bounding.boxes.push(scenery_bounds);
    }
}
