#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Loading,
    InGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameState {
    Playing,
    Paused,
}
