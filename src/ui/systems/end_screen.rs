use crate::{
    consts::{
        ASSET_FONTS_DEFAULT, COLOR_ACCENT, COLOR_BUTTON_DEFAULT, COLOR_FOREGROUND,
        COLOR_TRANSPARENT, ASSET_AUDIO_DEATH,
    },
    schedule::GameState,
    stats::Stats,
    ui::components::{EndScreenButtonAction, OnDeathScreen},
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_end_screen_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stats: Res<Stats>,
) {
    let font = asset_server.get_handle(ASSET_FONTS_DEFAULT);

    let button_style = Style {
        size: Size::new(Val::Percent(40.), Val::Percent(20.)),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 60.,
        color: COLOR_FOREGROUND,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Undefined, Val::Undefined),
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnDeathScreen)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "You died!",
                    TextStyle {
                        font: asset_server.get_handle(ASSET_FONTS_DEFAULT),
                        font_size: 120.0,
                        color: COLOR_ACCENT,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::new(
                        Val::Undefined,
                        Val::Undefined,
                        Val::Undefined,
                        Val::Px(50.0),
                    ),
                    ..default()
                })
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );

            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("Score: {}", stats.score),
                    TextStyle {
                        font: asset_server.get_handle(ASSET_FONTS_DEFAULT),
                        font_size: 100.0,
                        color: COLOR_ACCENT,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Undefined),
                    ..default()
                })
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );

            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("Kills: {}", stats.kills),
                    TextStyle {
                        font: asset_server.get_handle(ASSET_FONTS_DEFAULT),
                        font_size: 100.0,
                        color: COLOR_ACCENT,
                    },
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::new(
                        Val::Undefined,
                        Val::Undefined,
                        Val::Undefined,
                        Val::Px(50.0),
                    ),
                    ..default()
                })
                .with_text_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }),
            );

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(EndScreenButtonAction::Restart)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Restart",
                        button_text_style.clone(),
                    ));
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(EndScreenButtonAction::MainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Main Menu",
                        button_text_style.clone(),
                    ));
                });

            // Quit button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(EndScreenButtonAction::Quit)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });
}

pub fn end_screen_button_interaction_system(
    query: Query<(&Interaction, &EndScreenButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                EndScreenButtonAction::Restart => game_state.set(GameState::InGame).unwrap(),
                EndScreenButtonAction::MainMenu => game_state.set(GameState::MainMenu).unwrap(),
                EndScreenButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}

pub fn end_screen_death_sound(audio: Res<Audio>,
mut asset_server: Res<AssetServer>) {
    audio.play_with_settings(asset_server.load::<AudioSource, &str>(ASSET_AUDIO_DEATH), PlaybackSettings::ONCE.with_volume(0.1));
}