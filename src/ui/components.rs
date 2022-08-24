use bevy::prelude::*;

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct SplashScreenTimer(pub Timer);

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}
