use crate::schedule::{resources::ScheduleQueue, GameState, ScheduleTimer};
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

macro_rules! create_schedule_system {
    (
        $on_enter_name: ident,
        $on_update_name: ident,
        $on_exit_name: ident,
        $timer_time: expr,
        $game_state: expr,
    ) => {
        pub fn $on_enter_name(mut commands: Commands, mut schedule_queue: ResMut<ScheduleQueue>) {
            schedule_queue.0.push_back($game_state);
            commands.insert_resource(ScheduleTimer(Timer::from_seconds($timer_time, false)));
        }

        pub fn $on_update_name(
            mut game_state: ResMut<State<GameState>>,
            time: Res<Time>,
            mut timer: ResMut<ScheduleTimer>,
            mut schedule_queue: ResMut<ScheduleQueue>,
        ) {
            if timer.0.tick(time.delta()).just_finished() {
                if let Some(schedule) = schedule_queue.0.pop_front() {
                    game_state.set(schedule).unwrap();
                }
            }
        }

        pub fn $on_exit_name(mut commands: Commands) {
            commands.remove_resource::<ScheduleTimer>();
        }
    };
    (
        $on_enter_name: ident,
        $on_update_name: ident,
        $on_exit_name: ident,
        $timer_time: expr,
    ) => {
        pub fn $on_enter_name(mut commands: Commands) {
            commands.insert_resource(ScheduleTimer(Timer::from_seconds($timer_time, false)));
        }

        pub fn $on_update_name(
            mut game_state: ResMut<State<GameState>>,
            time: Res<Time>,
            mut timer: ResMut<ScheduleTimer>,
            mut schedule_queue: ResMut<ScheduleQueue>,
        ) {
            if timer.0.tick(time.delta()).just_finished() {
                if let Some(schedule) = schedule_queue.0.pop_front() {
                    game_state.set(schedule).unwrap();
                }
            }
        }

        pub fn $on_exit_name(mut commands: Commands) {
            commands.remove_resource::<ScheduleTimer>();
        }
    };
}

// Splash Screen
create_schedule_system! {
    on_enter_before_splash_screen_system,
    on_update_before_splash_screen_system,
    on_exit_before_splash_screen_system,
    1.0,
    GameState::SplashScreen,
}

create_schedule_system! {
    on_enter_after_splash_screen_system,
    on_update_after_splash_screen_system,
    on_exit_after_splash_screen_system,
    1.0,
}

// Main Menu
create_schedule_system! {
    on_enter_before_main_menu_system,
    on_update_before_main_menu_system,
    on_exit_before_main_menu_system,
    0.5,
    GameState::MainMenu,
}

create_schedule_system! {
    on_enter_after_main_menu_system,
    on_update_after_main_menu_system,
    on_exit_after_main_menu_system,
    0.5,
}

// In Game
create_schedule_system! {
    on_enter_before_in_game_system,
    on_update_before_in_game_system,
    on_exit_before_in_game_system,
    0.5,
    GameState::InGame,
}

create_schedule_system! {
    on_enter_after_in_game_system,
    on_update_after_in_game_system,
    on_exit_after_in_game_system,
    3.0,
}

// End Screen
create_schedule_system! {
    on_enter_before_end_screen_system,
    on_update_before_end_screen_system,
    on_exit_before_end_screen_system,
    0.5,
    GameState::EndScreen,
}

create_schedule_system! {
    on_enter_after_end_screen_system,
    on_update_after_end_screen_system,
    on_exit_after_end_screen_system,
    0.5,
}
