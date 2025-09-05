mod animation;
mod camera;
mod creature;
mod event;
mod player;
mod save;
mod team;
mod ui;
mod world;

use bevy::prelude::*;
use bevy::state::state::States;
use bevy::window::WindowResolution;
use bevy_ecs_ldtk::prelude::*;

use crate::animation::AnimationsPlugin;
use crate::camera::*;
use crate::creature::DexPlugin;
use crate::event::EventsPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;

use crate::world::WorldPlugin;

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

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pokeclone".to_string(),
                        resizable: false,
                        resolution: WindowResolution::new(1000.0, 600.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            LdtkPlugin,
            EventsPlugin,
            WorldPlugin,
            UiPlugin,
            CamPlugin,
            DexPlugin,
            PlayerPlugin,
            AnimationsPlugin,
        ))
        .init_state::<AppState>()
        .run();
}
