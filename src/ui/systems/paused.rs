use crate::{
    asset::FontHandles,
    schedule::{GameState, GotoMainMenu},
    ui::{
        components::{OnPausedScreen, PausedScreenButtonAction},
        helper::{default_button_bundle, default_node_bundle_style},
    },
    ui::{constants::COLOR_TRANSPARENT, helper::default_small_button_text_style},
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_paused_ui_system(mut commands: Commands, font_handles: Res<FontHandles>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Undefined, Val::Undefined),
                ..default_node_bundle_style()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnPausedScreen)
        .with_children(|parent| {
            // Continue button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(PausedScreenButtonAction::Continue)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Continue",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });

            // Main Menu button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(PausedScreenButtonAction::MainMenu)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Main Menu",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
                });

            // Quit button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(PausedScreenButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Quit",
                        default_small_button_text_style(font_handles.default.clone()),
                    ));
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
