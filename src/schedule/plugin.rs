use super::GameState;
use bevy::prelude::*;

/// A plugin for the schedule of the game.
pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::SplashScreen);
        // .add_state(MenuState::Disabled);
    }
}
