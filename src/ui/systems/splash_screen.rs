use crate::{
    asset::FontHandles,
    schedule::{GameState, ScheduleQueue},
    ui::{components::OnSplashScreen, helper::accent_huge_button_text_style},
};
use bevy::prelude::*;

pub fn spawn_splash_screen_system(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Escape Pod",
                accent_huge_button_text_style(font_handles.default.clone()),
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            })
            .with_text_alignment(TextAlignment::CENTER),
        )
        .insert(OnSplashScreen);
}

pub fn update_splash_screen_system(
    mut game_state: ResMut<State<GameState>>,
    mut schedule_queue: ResMut<ScheduleQueue>,
) {
    game_state.set(GameState::AfterSplashScreen).unwrap();
    schedule_queue.0.push_back(GameState::BeforeMainMenu);
}
