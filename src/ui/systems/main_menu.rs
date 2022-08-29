use crate::{
    asset::FontHandles,
    audio::AudioSettings,
    colors::COLOR_TRANSPARENT,
    schedule::{GameState, ScheduleQueue},
    ui::helper::default_node_bundle_style,
    ui::{
        components::{MainMenuButtonAction, OnMainMenuScreen, UiVolume},
        helper::{
            accent_large_button_text_style, default_button_bundle, default_small_button_text_style,
        },
    },
};
use bevy::{app::AppExit, prelude::*};

pub fn spawn_main_menu_ui_system(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    audio_settings: Res<AudioSettings>,
) {
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

            // Volume button
            parent
                .spawn_bundle(default_button_bundle())
                .insert(MainMenuButtonAction::Volume)
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
    mut audio_settings: ResMut<AudioSettings>,
) {
    for (interaction, action) in query.iter() {
        if *interaction == Interaction::Clicked {
            match action {
                MainMenuButtonAction::Play => {
                    game_state.set(GameState::AfterMainMenu).unwrap();
                    schedule_queue.0.push_back(GameState::BeforeInGame);
                }
                MainMenuButtonAction::Volume => audio_settings.toggle(),
                MainMenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}

pub fn update_ui_volume_system(
    audio_settings: Res<AudioSettings>,
    mut ui_query: Query<&mut Text, With<UiVolume>>,
) {
    let mut ui_volume = ui_query.single_mut();
    let section = &mut ui_volume.sections[1];
    section.value = format!("{}%", audio_settings.volume());
}
