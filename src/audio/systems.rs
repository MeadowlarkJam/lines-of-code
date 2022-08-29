use super::{
    resources::AudioTimer, AudioEvent, AudioSettings, AudioType, PriorityAudioEvent,
    PriorityAudioType,
};
use crate::asset::AudioHandles;
use bevy::{audio::AudioSink, prelude::*};

pub fn play_ingame_audio_system(
    mut audio_events: EventReader<AudioEvent>,
    audio: Res<Audio>,
    mut audio_handles: ResMut<AudioHandles>,
    time: Res<Time>,
    mut audio_timer: ResMut<AudioTimer>,
    audio_settings: ResMut<AudioSettings>,
) {
    // Limit the amount of time between each audio event.
    if audio_timer.0.tick(time.delta()).finished() {
        // Only play a single sound a frame to avoid overlapping sounds.
        if let Some(event) = audio_events.iter().next() {
            match event {
                AudioEvent(AudioType::Hit) => {
                    audio_handles.hit_sink = audio.play_with_settings(
                        audio_handles.hit.clone(),
                        PlaybackSettings::ONCE.with_volume(audio_settings.volume_f32()),
                    );
                }
                AudioEvent(AudioType::Laser) => {
                    audio_handles.laser_sink = audio.play_with_settings(
                        audio_handles.laser.clone(),
                        PlaybackSettings::ONCE.with_volume(audio_settings.volume_f32()),
                    );
                }
                AudioEvent(AudioType::Explosion) => {
                    audio_handles.explosion_sink = audio.play_with_settings(
                        audio_handles.explosion.clone(),
                        PlaybackSettings::ONCE.with_volume(audio_settings.volume_f32()),
                    );
                }
            }
        }
    }

    // Clear any audio events that are left.
    audio_events.clear();
}

pub fn play_priority_audio_system(
    mut audio_events: EventReader<PriorityAudioEvent>,
    audio: Res<Audio>,
    mut audio_handles: ResMut<AudioHandles>,
    audio_settings: ResMut<AudioSettings>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    for event in audio_events.iter() {
        match event {
            PriorityAudioEvent(PriorityAudioType::Intro) => {
                audio_handles.intro_sink = audio.play_with_settings(
                    audio_handles.intro.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_settings.volume_f32()),
                );
                audio_handles.intro_sink.make_strong(&audio_sinks);
            }
            PriorityAudioEvent(PriorityAudioType::Music) => {
                audio_handles.music_sink = audio.play_with_settings(
                    audio_handles.music.clone(),
                    PlaybackSettings::LOOP.with_volume(audio_settings.volume_f32()),
                );
                audio_handles.music_sink.make_strong(&audio_sinks);
            }
            PriorityAudioEvent(PriorityAudioType::Death) => {
                audio_handles.death_sink = audio.play_with_settings(
                    audio_handles.death.clone(),
                    PlaybackSettings::ONCE.with_volume(audio_settings.volume_f32()),
                );
                audio_handles.death_sink.make_strong(&audio_sinks);
            }
        }
    }

    audio_events.clear();
}

pub fn play_intro_audio_system(mut audio_events: EventWriter<PriorityAudioEvent>) {
    audio_events.send(PriorityAudioEvent(PriorityAudioType::Intro));
}

pub fn play_music_audio_system(mut audio_events: EventWriter<PriorityAudioEvent>) {
    audio_events.send(PriorityAudioEvent(PriorityAudioType::Music));
}

pub fn adjust_audio_volume_system(
    audio_handles: Res<AudioHandles>,
    mut audio_sinks: ResMut<Assets<AudioSink>>,
    audio_settings: Res<AudioSettings>,
) {
    if let Some(connect_sink) = audio_sinks.get_mut(&audio_handles.connect_sink) {
        connect_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(death_sink) = audio_sinks.get_mut(&audio_handles.death_sink) {
        death_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(hit_sink) = audio_sinks.get_mut(&audio_handles.hit_sink) {
        hit_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(laser_sink) = audio_sinks.get_mut(&audio_handles.laser_sink) {
        laser_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(explosion_sink) = audio_sinks.get_mut(&audio_handles.explosion_sink) {
        explosion_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(cannon_sink) = audio_sinks.get_mut(&audio_handles.cannon_sink) {
        cannon_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(intro_sink) = audio_sinks.get_mut(&audio_handles.intro_sink) {
        intro_sink.set_volume(audio_settings.volume_f32());
    }
    if let Some(music_sink) = audio_sinks.get_mut(&audio_handles.music_sink) {
        music_sink.set_volume(audio_settings.volume_f32());
    }
}
