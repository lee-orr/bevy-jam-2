#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum States {
    Loading,
    Menu,
    LoadingLevel,
    InGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameMode {
    None,
    Exploration,
    Conversation,
}
