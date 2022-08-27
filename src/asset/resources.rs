use bevy::prelude::*;

#[derive(Default)]
pub struct SoundHandles {
    pub connect: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub laser: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub cannon: Handle<AudioSource>,
}
