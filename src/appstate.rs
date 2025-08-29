use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
/// States of the game
pub enum AppState {
    /// `MainMenu` is the initial state, when the main menu UI is displayed.
    #[default]
    MainMenu,
    /// `ResumeGame` is a preliminary state to `InGame`. It loads the save before running the game.
    ResumeGame,
    /// `InGame` is the state when we can play. The world, team and actions UIs are displayed.
    InGame,
    /// `InFight` is when a battle occurs. The world is hidden, the fight is displayed instead, and the actions UI is updated.
    InFight,
    /// `OptionsMenu` is the state when the options UI is displayed.
    OptionsMenu,
}
