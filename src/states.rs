use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyAppState {
    LoadingScreen,
    MainMenu,
    InGame,
}
