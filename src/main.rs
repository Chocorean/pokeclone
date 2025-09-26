mod animation;
mod camera;
mod dex;
mod event;
mod fight;
mod player;
mod save;
mod team;
mod ui;
mod utils;
mod world;

use bevy::prelude::*;
use bevy::state::state::States;
use bevy::window::WindowResolution;
use bevy_easy_gif::prelude::GifPlugin;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

use crate::animation::AnimationsPlugin;
use crate::camera::{CamPlugin, WorldBundle};
use crate::dex::DexPlugin;
use crate::event::EventsPlugin;
use crate::fight::FightPlugin;
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
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pokeclone".to_string(),
                    resizable: true,
                    resolution: WindowResolution::new(1000.0, 600.0),
                    ..default()
                }),
                ..default()
            }),
    );
    app.add_plugins((
        LdtkPlugin,
        GifPlugin,
        EventsPlugin,
        WorldPlugin,
        UiPlugin,
        CamPlugin,
        DexPlugin,
        PlayerPlugin,
        AnimationsPlugin,
        FightPlugin,
    ));
    app.init_state::<AppState>();

    if cfg!(debug_assertions) {
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    }
    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        WorldBundle,
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("ldtk/world.ldtk").into(),
            ..Default::default()
        },
        // AudioPlayer::new(asset_server.load("sfx/town.flac")),
    ));
    commands.insert_resource(LevelSelection::index(0));
}
