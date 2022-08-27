use bevy::prelude::*;

pub struct Hit {
    pub damage: u32,
    pub target: Entity,
}

pub enum Sound {
    Connect,
    Hit,
    Zap,
}

pub struct SoundEvent(Sound);
