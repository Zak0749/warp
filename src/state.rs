#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Loading,
    Game,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauseState {
    Paused,
    Unpaused,
}
