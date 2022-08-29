use super::{
    resources::StatsTimer,
    systems::{reset_stats_system, update_stats_system},
    Stats,
};
use crate::schedule::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct StatsSystem;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Stats::default())
            .insert_resource(StatsTimer(Timer::from_seconds(3.0, true)))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(StatsSystem)
                    .with_system(update_stats_system),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .label(StatsSystem)
                    .with_system(reset_stats_system),
            );
    }
}
