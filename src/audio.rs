use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Menu), start_menu_music)
            .add_systems(OnEnter(GameState::Playing), start_frontyard_music);
    }
}

fn start_menu_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.kickstart.clone())
        .looped()
        .with_volume(0.3)
        .handle();
}

fn start_frontyard_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.stop();
    audio.play(audio_assets.no_rain.clone())
        .looped()
        .with_volume(0.3)
        .handle();
}
