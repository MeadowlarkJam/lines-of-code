/// A state of the game.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    AssetLoading,
    SplashScreen,
    MainMenu,
    InGame,
    Paused,
    EndScreen,
}
