use super::{resources::AudioTimer, AudioEvent, AudioType};
use crate::asset::AudioHandles;
use bevy::prelude::*;

pub fn play_ingame_audio_system(
    mut audio_events: EventReader<AudioEvent>,
    audio: Res<Audio>,
    audio_handles: Res<AudioHandles>,
    time: Res<Time>,
    mut audio_timer: ResMut<AudioTimer>,
) {
    // Limit the amount of time between each audio event.
    if !audio_timer.0.tick(time.delta()).finished() {
        return;
    }

    // Only play a single sound a frame to avoid overlapping sounds.
    if let Some(event) = audio_events.iter().next() {
        match event {
            AudioEvent(AudioType::Hit) => {
                audio.play_with_settings(
                    audio_handles.hit.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
            AudioEvent(AudioType::Laser) => {
                audio.play_with_settings(
                    audio_handles.laser.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
            AudioEvent(AudioType::Explosion) => {
                audio.play_with_settings(
                    audio_handles.explosion.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
        }
    }

    // Clear any audio events that are left.
    audio_events.clear();
}

pub fn play_intro_audio_system(audio: Res<Audio>, audio_handles: Res<AudioHandles>) {
    audio.play_with_settings(
        audio_handles.intro.clone(),
        PlaybackSettings::ONCE.with_volume(0.1),
    );
}

pub fn play_music_audio_system(audio: Res<Audio>, audio_handles: Res<AudioHandles>) {
    audio.play_with_settings(
        audio_handles.music.clone(),
        PlaybackSettings::LOOP.with_volume(0.2),
    );
}

pub fn play_death_audio_system(audio: Res<Audio>, audio_handles: Res<AudioHandles>) {
    audio.play_with_settings(
        audio_handles.death.clone(),
        PlaybackSettings::ONCE.with_volume(0.1),
    );
}
