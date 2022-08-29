/// A state of the game.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    AssetLoading,
    Paused,

    BeforeSplashScreen,
    SplashScreen,
    AfterSplashScreen,

    BeforeMainMenu,
    MainMenu,
    AfterMainMenu,

    BeforeInGame,
    InGame,
    AfterInGame,

    BeforeEndScreen,
    EndScreen,
    AfterEndScreen,
}
