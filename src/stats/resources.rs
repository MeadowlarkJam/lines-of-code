use bevy::prelude::*;

#[derive(Default)]
pub struct Stats {
    pub score: u32,
    pub kills: u32,
    pub enemies_alive: u32,
}

impl Stats {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub struct StatsTimer(pub Timer);
