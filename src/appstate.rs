use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    LoadGame,
    InGame,
    InGameMenu,
    Fight,
    OptionsMenu,
}
