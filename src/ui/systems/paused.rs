use crate::{
    consts::{ASSET_FONTS_DEFAULT, COLOR_BUTTON_DEFAULT, COLOR_FOREGROUND, COLOR_TRANSPARENT},
    schedule::{GameState, GotoMainMenu},
    ui::components::{OnPausedScreen, PausedScreenButtonAction},
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_paused_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.get_handle(ASSET_FONTS_DEFAULT);

    let button_style = Style {
        size: Size::new(Val::Px(400.0), Val::Px(100.0)),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font,
        font_size: 80.0,
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
        .insert(OnPausedScreen)
        .with_children(|parent| {
            // Continue button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(PausedScreenButtonAction::Continue)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Continue",
                        button_text_style.clone(),
                    ));
                });

            // Main Menu button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: COLOR_BUTTON_DEFAULT.into(),
                    ..default()
                })
                .insert(PausedScreenButtonAction::MainMenu)
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
                .insert(PausedScreenButtonAction::Quit)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });
}

#[allow(clippy::type_complexity)]
pub fn paused_button_interaction_system(
    query: Query<(&Interaction, &PausedScreenButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut event_writer: EventWriter<GotoMainMenu>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                PausedScreenButtonAction::Continue => game_state.pop().unwrap(),
                PausedScreenButtonAction::MainMenu => {
                    event_writer.send(GotoMainMenu);
                    game_state.pop().unwrap();
                }
                PausedScreenButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
