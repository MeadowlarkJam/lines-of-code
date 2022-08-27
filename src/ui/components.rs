use bevy::prelude::*;

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnIngameScreen;

#[derive(Component)]
pub struct OnPausedScreen;

#[derive(Component)]
pub struct OnDeathScreen;

#[derive(Component)]
pub struct UiHealth;

#[derive(Component)]
pub struct UiSize;

#[derive(Component)]
pub struct UiScore;

#[derive(Component)]
pub struct UiKills;

#[derive(Component)]
pub struct UiEnemiesAlive;

#[derive(Component)]
pub struct SplashScreenTimer(pub Timer);

#[derive(Component)]
pub enum MainMenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
pub enum PausedScreenButtonAction {
    Continue,
    MainMenu,
    Quit,
}

#[derive(Component)]
pub enum EndScreenButtonAction {
    Restart,
    MainMenu,
    Quit,
}
