use crate::{
    consts::ASSET_FONTS_DEFAULT,
    schedule::GameState,
    ui::components::{OnSplashScreen, SplashScreenTimer},
};
use bevy::prelude::*;

pub fn spawn_splash_screen_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Escape Pod",
                TextStyle {
                    font: asset_server.get_handle(ASSET_FONTS_DEFAULT),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            })
            .with_text_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }),
        )
        .insert(OnSplashScreen);

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(1.0, false)));
}

pub fn update_splash_screen_system(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashScreenTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu).unwrap();
    }
}
