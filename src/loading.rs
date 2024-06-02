use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(path = "audio/kickstart.wav")]
    pub kickstart: Handle<AudioSource>,
    #[asset(path = "audio/no_rain.wav")]
    pub no_rain: Handle<AudioSource>,
    #[asset(path = "audio/galaxie.wav")]
    pub galaxie: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,

    #[asset(path = "textures/grass.png")]
    pub grass: Handle<Image>,

    #[asset(path = "textures/asphalt1.png")]
    pub asphalt: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 47., columns = 6, rows = 4))]
    pub sprite_layout: Handle<TextureAtlasLayout>,

    #[asset(image(sampler = nearest))]
    #[asset(path = "textures/sprite_walk.png")]
    pub sprite_walk: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 48., tile_size_y = 48., columns = 2, rows = 4))]
    pub driveway_layout: Handle<TextureAtlasLayout>,

    #[asset(
        paths(
            "textures/asphalt1.png",
            "textures/asphalt2.png",
            "textures/asphalt3.png",
            "textures/asphalt4.png",
            "textures/asphalt5.png",
            "textures/asphalt6.png",
            "textures/asphalt7.png",
            "textures/asphalt8.png"
        ),
        collection(typed)
    )]
    pub driveway: Vec<Handle<Image>>,

    #[asset(path = "textures/tree1.png")]
    pub tree1: Handle<Image>,

    #[asset(path = "textures/car_right_blue.png")]
    pub car_right_blue: Handle<Image>,

    #[asset(path = "textures/car_right_red.png")]
    pub car_right_red: Handle<Image>,

    #[asset(path = "textures/car_right_gray.png")]
    pub car_right_gray: Handle<Image>,

    #[asset(path = "textures/house.png")]
    pub house: Handle<Image>,
}
