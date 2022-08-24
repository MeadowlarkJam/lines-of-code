use crate::{
    consts::{ASSET_FONTS_DEFAULT, BUTTON_COLOR},
    schedule::GameState,
    ui::components::{MenuButtonAction, OnMainMenuScreen},
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_main_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.get_handle(ASSET_FONTS_DEFAULT);

    let button_style = Style {
        size: Size::new(Val::Percent(80.0), Val::Px(75.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Game name
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Escape Pod",
                    TextStyle {
                        font,
                        font_size: 80.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            // Play button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Play", button_text_style.clone()));
                });

            // Quit button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });
}

pub fn update_main_menu_system(
    query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                MenuButtonAction::Play => game_state.set(GameState::InGame).unwrap(),
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
