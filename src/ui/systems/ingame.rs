use crate::{
    asset::FontHandles,
    components::Properties,
    player::PlayerRoot,
    stats::Stats,
    ui::components::{OnIngameScreen, UiEnemiesAlive, UiHealth, UiKills, UiScore, UiSize},
    ui::{
        constants::COLOR_TRANSPARENT,
        helper::{default_node_bundle_style, default_small_button_text_style},
    },
};
use bevy::prelude::*;

pub fn spawn_ingame_ui_system(mut commands: Commands, font_handles: Res<FontHandles>) {
    let text_style = default_small_button_text_style(font_handles.default.clone());
    let style = Style {
        margin: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(5.0), Val::Px(5.0)),
        align_self: AlignSelf::FlexStart,
        ..default()
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position: UiRect::new(Val::Px(25.0), Val::Undefined, Val::Px(25.0), Val::Undefined),
                align_self: AlignSelf::FlexStart,
                ..default_node_bundle_style()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnIngameScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new("Score: ", text_style.clone()),
                        TextSection::from_style(text_style.clone()),
                    ])
                    .with_style(style.clone()),
                )
                .insert(OnIngameScreen)
                .insert(UiScore);

            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new("Kills: ", text_style.clone()),
                        TextSection::from_style(text_style.clone()),
                    ])
                    .with_style(style.clone()),
                )
                .insert(OnIngameScreen)
                .insert(UiKills);

            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new("Enemies: ", text_style.clone()),
                        TextSection::from_style(text_style.clone()),
                    ])
                    .with_style(style.clone()),
                )
                .insert(OnIngameScreen)
                .insert(UiEnemiesAlive);
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position: UiRect::new(Val::Px(25.0), Val::Undefined, Val::Undefined, Val::Px(25.0)),
                ..default_node_bundle_style()
            },
            color: COLOR_TRANSPARENT.into(),
            ..default()
        })
        .insert(OnIngameScreen)
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new("Health: ", text_style.clone()),
                        TextSection::from_style(text_style.clone()),
                    ])
                    .with_style(style.clone()),
                )
                .insert(OnIngameScreen)
                .insert(UiHealth);

            parent
                .spawn_bundle(
                    TextBundle::from_sections([
                        TextSection::new("Size: ", text_style.clone()),
                        TextSection::from_style(text_style.clone()),
                    ])
                    .with_style(style.clone()),
                )
                .insert(OnIngameScreen)
                .insert(UiSize);
        });
}

pub fn update_ui_score_system(stats: Res<Stats>, mut ui_query: Query<&mut Text, With<UiScore>>) {
    let mut ui_score = ui_query.single_mut();
    let section = &mut ui_score.sections[1];
    section.value = format!("{}", stats.score);
}

pub fn update_ui_kills_system(stats: Res<Stats>, mut ui_query: Query<&mut Text, With<UiKills>>) {
    let mut ui_kills = ui_query.single_mut();
    let section = &mut ui_kills.sections[1];
    section.value = format!("{}", stats.kills);
}

pub fn update_ui_enemies_alive_system(
    stats: Res<Stats>,
    mut ui_query: Query<&mut Text, With<UiEnemiesAlive>>,
) {
    let mut ui_enemies_alive = ui_query.single_mut();
    let section = &mut ui_enemies_alive.sections[1];
    section.value = format!("{}", stats.enemies_alive);
}

pub fn update_ui_player_stats_system(
    player_query: Query<&Properties, With<PlayerRoot>>,
    mut ui_health_query: Query<&mut Text, (With<UiHealth>, Without<UiSize>)>,
    mut ui_size_query: Query<&mut Text, (With<UiSize>, Without<UiHealth>)>,
) {
    let properties = player_query.single();
    let mut ui_health = ui_health_query.single_mut();
    let mut ui_size = ui_size_query.single_mut();

    ui_health.sections[1].value = format!("{}", properties.health);
    ui_size.sections[1].value = format!("{}", properties.size);
}
