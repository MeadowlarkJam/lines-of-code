use crate::{
    consts::{ASSET_AUDIO_BG_SONG, ASSET_FONTS_DEFAULT, COLOR_ACCENT},
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
                    font_size: 140.0,
                    color: COLOR_ACCENT,
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

pub fn start_music(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio.play_with_settings(
        asset_server.load(ASSET_AUDIO_BG_SONG),
        PlaybackSettings::LOOP.with_volume(0.2),
    );
}
