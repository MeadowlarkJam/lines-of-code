/// A state of the game.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    SplashScreen,
    MainMenu,
    InGame,
    Paused,
}
