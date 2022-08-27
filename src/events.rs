use bevy::prelude::*;

pub struct Hit {
    pub damage: u32,
    pub target: Entity,
}

pub enum Sound {
    Connect,
    Hit,
    Zap,
    Death,
    CannonShot,
}

pub struct SoundEvent(pub Sound);
