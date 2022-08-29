use crate::{
    asset::FontHandles,
    schedule::GameState,
    stats::Stats,
    ui::{
        components::{EndScreenButtonAction, OnDeathScreen},
        constants::COLOR_TRANSPARENT,
        helper::{
            accent_large_button_text_style, accent_medium_button_text_style, default_button_bundle,
            default_node_bundle_style, default_small_button_text_style,
        },
    },
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_end_screen_ui_system(
    mut commands: Commands,
    stats: Res<Stats>,
    font_handles: Res<FontHandles>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Undefined, Val::Undefined),
                ..default_node_bundle_style()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnDeathScreen)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "You died!",
                    accent_large_button_text_style(font_handles.default.clone()),
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
                .with_text_alignment(TextAlignment::CENTER),
            );

            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("Score: {}", stats.score),
                    accent_medium_button_text_style(font_handles.default.clone()),
                )
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    ..default()
                })
                .with_text_alignment(TextAlignment::CENTER),
            );

            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("Kills: {}", stats.kills),
                    accent_medium_button_text_style(font_handles.default.clone()),
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
                .with_text_alignment(TextAlignment::CENTER),
            );

            // Restart button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(EndScreenButtonAction::Restart)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Restart",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });

            // Main Menu button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(EndScreenButtonAction::MainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Main Menu",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });

            // Quit button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(EndScreenButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Quit",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });
        });
}

#[allow(clippy::type_complexity)]
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
