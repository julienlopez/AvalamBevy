use bevy::prelude::{Resource, States};

pub struct Score {
    pub red: u8,
    pub yellow: u8,
}

#[derive(Resource)]
pub struct FinalScore {
    pub score: Option<Score>,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    EndPanel,
}
