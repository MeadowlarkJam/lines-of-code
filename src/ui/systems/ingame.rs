use crate::{
    components::Properties,
    consts::{ASSET_FONTS_DEFAULT, COLOR_BACKGROUND_DARK, COLOR_FOREGROUND},
    player::PlayerRoot,
    stats::Stats,
    ui::components::{OnIngameScreen, UiEnemiesAlive, UiHealth, UiKills, UiScore, UiSize},
};
use bevy::prelude::*;

pub fn spawn_ingame_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.get_handle(ASSET_FONTS_DEFAULT);

    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: COLOR_FOREGROUND,
    };

    let style = Style {
        margin: UiRect::new(Val::Px(25.0), Val::Px(25.0), Val::Px(10.0), Val::Px(10.0)),
        align_self: AlignSelf::FlexStart,
        ..default()
    };

    let node_style = Style {
        position_type: PositionType::Absolute,
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        align_self: AlignSelf::FlexStart,
        ..default()
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position: UiRect::new(Val::Px(25.0), Val::Undefined, Val::Px(25.0), Val::Undefined),
                ..node_style
            },
            color: COLOR_BACKGROUND_DARK.into(),
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
                ..node_style
            },
            color: COLOR_BACKGROUND_DARK.into(),
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
