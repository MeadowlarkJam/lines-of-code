use super::{
    systems::{check_for_paused_system, check_for_state_events_system, check_for_unpaused_system},
    GameState, GotoMainMenu,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct ScheduleSystem;

/// A plugin for the schedule of the game.
pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::AssetLoading)
            .add_event::<GotoMainMenu>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(ScheduleSystem)
                    .with_system(check_for_state_events_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(ScheduleSystem)
                    .with_system(check_for_paused_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .label(ScheduleSystem)
                    .with_system(check_for_unpaused_system),
            );
    }
}
