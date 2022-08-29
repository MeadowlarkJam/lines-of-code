use super::{GameState, GotoMainMenu};
use bevy::prelude::*;

pub fn check_for_paused_system(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        input.clear_just_pressed(KeyCode::Escape);
        game_state.push(GameState::Paused).unwrap();
    }
}

pub fn check_for_unpaused_system(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        input.clear_just_pressed(KeyCode::Escape);
        game_state.pop().unwrap();
    }
}

// Workaround to be able to use `SystemSet::on_exit(GameState::InGame)` reliably.
// When going to the main menu with `game_state.set(GameState::MainMenu)` while the
// game is paused using `game_state.push(GameState::Pause)` the original `GameState::InGame`
// won't get `exit`ed which causes the clean up systems to not be executed. Instead we
// now use `game_state.pop()` to get back to `GameState::InGame` and write the `GotoMainMenu`
// event that gets read during the `on_update` of `GameState::InGame` which in turn moves us
// to the `GameState::MainMenu`. This way we get reliable exits of the `GameState::InGame`
// which means the cleanup system run exactly when we think they would.
pub fn check_for_state_events_system(
    event_reader: EventReader<GotoMainMenu>,
    mut game_state: ResMut<State<GameState>>,
) {
    if !event_reader.is_empty() {
        game_state.set(GameState::MainMenu).unwrap();
    }
    event_reader.clear();
}
