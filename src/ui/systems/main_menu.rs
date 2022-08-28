use crate::{
    consts::{
        ASSET_FONTS_DEFAULT, COLOR_ACCENT, COLOR_BUTTON_DEFAULT, COLOR_FOREGROUND,
        COLOR_TRANSPARENT, ASSET_AUDIO_BG_SONG,
    },
    schedule::GameState,
    ui::components::{MainMenuButtonAction, OnMainMenuScreen},
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_main_menu_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.get_handle(ASSET_FONTS_DEFAULT);

    let button_style = Style {
        size: Size::new(Val::Percent(40.), Val::Percent(30.)),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 80.0,
        color: COLOR_FOREGROUND,
    };

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
        .insert(OnMainMenuScreen);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexStart,
                size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                ..default()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Play button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(MainMenuButtonAction::Play)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Play", button_text_style.clone()));
                });

            // Quit button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(MainMenuButtonAction::Quit)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });
}

pub fn main_menu_button_interaction_system(
    query: Query<(&Interaction, &MainMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                MainMenuButtonAction::Play => game_state.set(GameState::InGame).unwrap(),
                MainMenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}

