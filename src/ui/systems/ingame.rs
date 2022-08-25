use crate::{
    components::Stats,
    consts::{ASSET_FONTS_DEFAULT, COLOR_FOREGROUND},
    player::PlayerRoot,
    schedule::GameState,
    ui::components::{OnIngameScreen, UiHealth, UiScore},
};
use bevy::prelude::*;

pub fn spawn_ingame_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.get_handle(ASSET_FONTS_DEFAULT);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: COLOR_FOREGROUND,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: COLOR_FOREGROUND,
                }),
            ])
            .with_style(Style {
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(25.0), Val::Px(25.0)),
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(OnIngameScreen)
        .insert(UiScore);

    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "Health: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: COLOR_FOREGROUND,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: COLOR_FOREGROUND,
                }),
            ])
            .with_style(Style {
                position: UiRect {
                    left: Val::Px(25.0),
                    bottom: Val::Px(15.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            }),
        )
        .insert(OnIngameScreen)
        .insert(UiHealth);
}

pub fn update_ui_score_system(
    player_query: Query<&Stats, With<PlayerRoot>>,
    mut ui_query: Query<&mut Text, With<UiScore>>,
) {
    let stats = player_query.single();
    let mut ui_score = ui_query.single_mut();

    let section = &mut ui_score.sections[1];
    section.value = format!("{}", stats.size - 1);
}

pub fn update_ui_health_system(
    player_query: Query<&Stats, With<PlayerRoot>>,
    mut ui_query: Query<&mut Text, With<UiHealth>>,
) {
    let stats = player_query.single();
    let mut ui_score = ui_query.single_mut();

    let section = &mut ui_score.sections[1];
    section.value = format!("{}", stats.health);
}

pub fn check_for_paused_system(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        input.clear_just_pressed(KeyCode::Escape);
        game_state.push(GameState::Paused).unwrap();
    }
}
