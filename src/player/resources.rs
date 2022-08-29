use bevy::prelude::*;

pub struct PlayerHistory {
    pub target_position: Vec3,
    pub new_position: Vec3,
    pub timer: Timer,
}

impl PlayerHistory {
    pub fn update_position(&mut self, new_position: Vec3) {
        self.target_position = self.new_position;
        self.new_position = new_position;
    }
}
