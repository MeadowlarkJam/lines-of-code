use super::{resources::StatsTimer, Stats};
use crate::{
    enemy::{EnemyKilled, EnemySpawned},
    player::PlayerSizeIncreased,
};
use bevy::prelude::*;

pub fn update_stats_system(
    time: Res<Time>,
    mut stats: ResMut<Stats>,
    mut score_timer: ResMut<StatsTimer>,

    enemy_killed_events: EventReader<EnemyKilled>,
    enemy_spawned_events: EventReader<EnemySpawned>,
    player_size_increased_events: EventReader<PlayerSizeIncreased>,
) {
    if score_timer.0.tick(time.delta()).just_finished() {
        stats.score += 10;
    }

    let player_size_increase = player_size_increased_events.len() as u32;
    player_size_increased_events.clear();

    if player_size_increase > 0 {
        stats.score += player_size_increase * 20;
    }

    let enemies_spawned = enemy_spawned_events.len() as u32;
    enemy_spawned_events.clear();

    if enemies_spawned > 0 {
        stats.enemies_alive += enemies_spawned;
    }

    let enemies_killed = enemy_killed_events.len() as u32;
    enemy_killed_events.clear();

    if enemies_killed > 0 {
        stats.enemies_alive -= enemies_killed;
        stats.kills += enemies_killed;
        stats.score += enemies_killed * 100;
    }
}

pub fn reset_stats_system(mut stats: ResMut<Stats>, mut timer: ResMut<StatsTimer>) {
    timer.0.reset();
    stats.reset();
}
