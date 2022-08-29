use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct FontHandles {
    pub default: Handle<Font>,
}

#[derive(Default)]
pub struct SpriteHandles {
    pub debris: Handle<Image>,
    pub zapper: Handle<Image>,
    pub player: Handle<Image>,
    pub shield: Handle<Image>,
    pub forcefield: Handle<Image>,
    pub cannon: Handle<Image>,
}

#[derive(Default)]
pub struct AudioHandles {
    pub connect: Handle<AudioSource>,
    pub death: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub laser: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub cannon: Handle<AudioSource>,
    pub intro: Handle<AudioSource>,
    pub music: Handle<AudioSource>,
}

#[derive(Default)]
pub struct LoadingAssets {
    handles: Vec<HandleUntyped>,
}

impl Deref for LoadingAssets {
    type Target = Vec<HandleUntyped>;

    fn deref(&self) -> &Self::Target {
        &self.handles
    }
}

impl DerefMut for LoadingAssets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handles
    }
}
