use crate::loading::TextureAssets;
use crate::GameState;
use bevy::math::bounding::{self, Aabb2d};
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

#[derive(Component)]
pub struct Bounding {
    pub boxes: Vec<Aabb2d>,
}

impl Bounding {
    fn new() -> Self {
        Bounding { boxes: Vec::new() }
    }
}

impl Scenery {
    fn from_texture(
        texture: Handle<Image>,
        trans_x: f32,
        trans_y: f32,
        trans_z: f32,
    ) -> (SpriteBundle, Bounding) {
        (
            SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(trans_x, trans_y, trans_z)),
                ..Default::default()
            },
            Bounding::new(),
        )
    }

    fn tiled_from_texture(
        texture: Handle<Image>,
        trans_x: f32,
        trans_y: f32,
        trans_z: f32,
        tile_x: bool,
        tile_y: bool,
        stretch_factor: f32,
    ) -> (SpriteBundle, ImageScaleMode, Bounding) {
        (
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(trans_x, trans_y, trans_z),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(
                        if tile_x { stretch_factor } else { 1.0 },
                        if tile_y { stretch_factor } else { 1.0 },
                        1.0,
                    ),
                },
                ..Default::default()
            },
            ImageScaleMode::Tiled {
                tile_x,
                tile_y,
                stretch_value: 1.0 / stretch_factor,
            },
            Bounding::new(),
        )
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
            parent.spawn(Scenery::tiled_from_texture(
                textures.tree1.clone(),
                440.0 / 0.5,
                0.0,
                0.0,
                false,
                true,
                15.0,
            ));
            // Cars
            parent.spawn(Scenery::from_texture(
                textures.car_right_gray.clone(),
                340.0 / 0.5,
                -600.0 / 0.5,
                0.0,
            ));
            parent.spawn(Scenery::from_texture(
                textures.car_right_blue.clone(),
                340.0 / 0.5,
                -500.0 / 0.5,
                0.0,
            ));
            parent.spawn(Scenery::from_texture(
                textures.car_right_red.clone(),
                340.0 / 0.5,
                -400.0 / 0.5,
                0.0,
            ));
            // House
            parent.spawn(Scenery::from_texture(
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
            parent.spawn(Scenery::tiled_from_texture(
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
