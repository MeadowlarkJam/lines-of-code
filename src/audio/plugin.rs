use super::{
    events::AudioEvent,
    resources::AudioTimer,
    systems::{
        play_ingame_audio_system, play_intro_audio_system, play_music_audio_system,
        play_priority_audio_system,
    },
    PriorityAudioEvent,
};
use crate::schedule::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct AudioSystem;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AudioEvent>()
            .add_event::<PriorityAudioEvent>()
            .insert_resource(AudioTimer(Timer::from_seconds(0.05, true)))
            .add_system(play_priority_audio_system)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(AudioSystem)
                    .with_system(play_ingame_audio_system),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::BeforeSplashScreen)
                    .label(AudioSystem)
                    .with_system(play_intro_audio_system)
                    .with_system(play_music_audio_system),
            );
    }
}
