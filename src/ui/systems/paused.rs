use crate::{
    asset::FontHandles,
    audio::AudioSettings,
    colors::COLOR_TRANSPARENT,
    schedule::{GameState, ScheduleQueue},
    ui::helper::default_small_button_text_style,
    ui::{
        components::{OnPausedScreen, PausedScreenButtonAction, UiVolume},
        helper::{default_button_bundle, default_node_bundle_style},
    },
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_paused_ui_system(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    audio_settings: Res<AudioSettings>,
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

            // Volume button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(PausedScreenButtonAction::Volume)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle::from_sections([
                            TextSection::new(
                                "Volume: ",
                                default_small_button_text_style(font_handles.default.clone()),
                            ),
                            TextSection::new(
                                format!("{}%", audio_settings.volume()),
                                default_small_button_text_style(font_handles.default.clone()),
                            ),
                        ]))
                        .insert(UiVolume);
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
    mut schedule_queue: ResMut<ScheduleQueue>,
    mut audio_settings: ResMut<AudioSettings>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                PausedScreenButtonAction::Continue => game_state.pop().unwrap(),
                PausedScreenButtonAction::Volume => audio_settings.toggle(),
                PausedScreenButtonAction::MainMenu => {
                    game_state.set(GameState::AfterInGame).unwrap();
                    schedule_queue.0.push_back(GameState::BeforeMainMenu);
                }
                PausedScreenButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
