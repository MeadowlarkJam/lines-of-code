use crate::{
    consts::{
        ASSET_AUDIO_DEATH, ASSET_AUDIO_EXPLOSION, ASSET_AUDIO_HIT, ASSET_AUDIO_LASER,
        ASSET_AUDIO_LOAD, ASSET_FONTS_DEFAULT, ASSET_SPRITES_CANNON, ASSET_SPRITES_DEBRIS,
        ASSET_SPRITES_FORCEFIELD, ASSET_SPRITES_PLAYER, ASSET_SPRITES_SHIELD, ASSET_SPRITES_ZAPPER,
        MAX_AMOUNT_OF_SOUNDS_PER_FRAME,
    },
    events::{Sound, SoundEvent},
};
use bevy::prelude::*;

use super::resources::SoundHandles;

pub fn load_ingame_assets_system(
    asset_server: Res<AssetServer>,
    mut sound_handles: ResMut<SoundHandles>,
) {
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_DEBRIS);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_ZAPPER);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_PLAYER);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_SHIELD);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_FORCEFIELD);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_CANNON);

    sound_handles.death = asset_server.load::<AudioSource, &str>(ASSET_AUDIO_DEATH);
    sound_handles.laser = asset_server.load::<AudioSource, &str>(ASSET_AUDIO_LASER);
    sound_handles.explosion = asset_server.load::<AudioSource, &str>(ASSET_AUDIO_EXPLOSION);
    sound_handles.hit = asset_server.load::<AudioSource, &str>(ASSET_AUDIO_HIT);
    sound_handles.loadup = asset_server.load::<AudioSource, &str>(ASSET_AUDIO_LOAD);
}

pub fn load_splash_sound(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_with_settings(
        asset_server.load::<AudioSource, &str>(ASSET_AUDIO_LOAD),
        PlaybackSettings::ONCE.with_volume(0.1),
    );
}

pub fn load_ui_assets_system(asset_server: Res<AssetServer>) {
    let _ = asset_server.load::<Font, &str>(ASSET_FONTS_DEFAULT);
}

pub fn play_sounds(
    mut sound_events: EventReader<SoundEvent>,
    audio: Res<Audio>,
    sound_handles: ResMut<SoundHandles>,
) {
    for (i, event) in sound_events.iter().enumerate() {
        if i >= MAX_AMOUNT_OF_SOUNDS_PER_FRAME {
            break;
        }

        match event {
            SoundEvent(Sound::Hit) => {
                audio.play_with_settings(
                    sound_handles.hit.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
            SoundEvent(Sound::Zap) => {
                audio.play_with_settings(
                    sound_handles.laser.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
            SoundEvent(Sound::Death) => {
                audio.play_with_settings(
                    sound_handles.death.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
            SoundEvent(Sound::CannonShot) => {
                audio.play_with_settings(
                    sound_handles.explosion.clone(),
                    PlaybackSettings::ONCE.with_volume(0.1),
                );
            }
        }
    }

    sound_events.clear();
}
