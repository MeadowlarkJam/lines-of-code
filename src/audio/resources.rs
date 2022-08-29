use bevy::prelude::*;

#[derive(Debug)]
pub struct AudioTimer(pub Timer);

#[derive(Debug)]
pub struct AudioSettings {
    volume: u8,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { volume: 100 }
    }
}

impl AudioSettings {
    pub fn toggle(&mut self) {
        if self.volume >= 100 {
            self.volume = 0;
        } else {
            self.volume += 10;
        }
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn volume_f32(&self) -> f32 {
        self.volume as f32 / 1000.0
    }
}
