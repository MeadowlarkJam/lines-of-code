use crate::{
    asset::FontHandles,
    schedule::GameState,
    ui::{
        components::{OnSplashScreen, SplashScreenTimer},
        helper::accent_large_button_text_style,
    },
};
use bevy::prelude::*;

pub fn spawn_splash_screen_system(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Escape Pod",
                accent_large_button_text_style(font_handles.default.clone()),
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            })
            .with_text_alignment(TextAlignment::CENTER),
        )
        .insert(OnSplashScreen);

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(3.0, false)));
}

pub fn update_splash_screen_system(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashScreenTimer>,
    mut query: Query<&mut Text, With<OnSplashScreen>>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    if timer.0.tick(time.delta()).finished()
        || keyboard.just_pressed(KeyCode::Escape)
        || mouse.just_pressed(MouseButton::Left)
    {
        game_state.set(GameState::MainMenu).unwrap();
    }

    let alpha = timer.0.elapsed().as_secs_f32() / timer.0.duration().as_secs_f32();

    for mut text in &mut query {
        for section in &mut text.sections {
            section.style.color.set_a(alpha);
        }
    }
}
