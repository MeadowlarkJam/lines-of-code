use super::{resources::StatsTimer, Stats};
use crate::{enemy::EnemyDied, player::PlayerSizeIncreased};
use bevy::prelude::*;

pub fn update_stats_system(
    time: Res<Time>,
    mut stats: ResMut<Stats>,
    mut score_timer: ResMut<StatsTimer>,
    enemy_died_events: EventReader<EnemyDied>,
    player_size_increased_events: EventReader<PlayerSizeIncreased>,
) {
    if score_timer.0.tick(time.delta()).just_finished() {
        stats.score += 10;
    }

    let enemies_died = enemy_died_events.len() as u32;
    enemy_died_events.clear();

    if enemies_died > 0 {
        stats.enemies_alive -= enemies_died;
        stats.kills += enemies_died;
        stats.score += enemies_died * 100;
    }

    let player_size_increase = player_size_increased_events.len() as u32;
    player_size_increased_events.clear();

    if player_size_increase > 0 {
        stats.score += player_size_increase * 20;
    }
}

pub fn reset_stats_system(mut stats: ResMut<Stats>, mut timer: ResMut<StatsTimer>) {
    timer.0.reset();
    stats.reset();
}
