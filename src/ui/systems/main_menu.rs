use crate::{
    asset::FontHandles,
    schedule::{GameState, ScheduleQueue},
    ui::{
        components::{MainMenuButtonAction, OnMainMenuScreen},
        helper::{
            accent_large_button_text_style, default_button_bundle, default_small_button_text_style,
        },
    },
    ui::{constants::COLOR_TRANSPARENT, helper::default_node_bundle_style},
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_main_menu_ui_system(mut commands: Commands, font_handles: Res<FontHandles>) {
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
        .insert(OnMainMenuScreen);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position: UiRect::new(
                    Val::Undefined,
                    Val::Undefined,
                    Val::Percent(60.0),
                    Val::Undefined,
                ),
                size: Size::new(Val::Percent(100.0), Val::Undefined),
                ..default_node_bundle_style()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Play button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(MainMenuButtonAction::Play)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Play",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });

            // Quit button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(MainMenuButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Quit",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });
        });
}

#[allow(clippy::type_complexity)]
pub fn main_menu_button_interaction_system(
    query: Query<(&Interaction, &MainMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut schedule_queue: ResMut<ScheduleQueue>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                MainMenuButtonAction::Play => {
                    game_state.set(GameState::AfterMainMenu).unwrap();
                    schedule_queue.0.push_back(GameState::BeforeInGame);
                }
                MainMenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
