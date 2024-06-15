use bevy::asset::Handle;
use bevy::math::{Quat, Vec3};
use bevy::prelude::Component;
use bevy::render::texture::Image;
use bevy::math::bounding::Aabb2d;
use bevy::sprite::{ImageScaleMode, SpriteBundle};
use bevy::transform::components::Transform;

#[derive(Component)]
pub struct Bounding {
    pub boxes: Vec<Aabb2d>,
}

impl Bounding {
    pub fn new() -> Self {
        Bounding { boxes: Vec::new() }
    }
}

pub struct Create;

impl Create {
    pub fn sprite(
        texture: Handle<Image>,
        trans_x: f32,
        trans_y: f32,
        trans_z: f32,
    ) -> SpriteBundle {
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(trans_x, trans_y, trans_z)),
            ..Default::default()
        }
    }

    pub fn bounded_sprite(
        texture: Handle<Image>,
        trans_x: f32,
        trans_y: f32,
        trans_z: f32,
    ) -> (SpriteBundle, Bounding) {
        (
            Self::sprite(texture, trans_x, trans_y, trans_z),
            Bounding::new(),
        )
    }

    pub fn tiled_sprite(
        texture: Handle<Image>,
        trans_x: f32,
        trans_y: f32,
        trans_z: f32,
        tile_x: bool,
        tile_y: bool,
        stretch_factor: f32,
    ) -> (SpriteBundle, ImageScaleMode) {
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
            }
        )
    }

    pub fn tiled_bounded_sprite(
        texture: Handle<Image>,
        trans_x: f32,
        trans_y: f32,
        trans_z: f32,
        tile_x: bool,
        tile_y: bool,
        stretch_factor: f32,
    ) -> (SpriteBundle, ImageScaleMode, Bounding) {
        let (sprite_bundle, image_scale_mode_tiled) = Self::tiled_sprite(
            texture, 
            trans_x, trans_y, trans_z, 
            tile_x, tile_y, stretch_factor
        );
        
        (sprite_bundle, image_scale_mode_tiled, Bounding::new())
    }
}