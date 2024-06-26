#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod ground;
mod loading;
mod menu;
mod player;
mod scenery;
mod helpe;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::ground::GroundPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::scenery::SceneryPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            GroundPlugin,
            SceneryPlugin,
        ));

        // #[cfg(debug_assertions)]
        {
            app.add_plugins(LogDiagnosticsPlugin::default());
        }
    }
}
