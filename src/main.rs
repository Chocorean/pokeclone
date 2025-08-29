use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_ldtk::prelude::*;

mod appstate;
use appstate::AppState;

mod camera;
use crate::camera::*;

mod character;
use crate::character::*;

mod ui;
use crate::creature::DexPlugin;
use crate::ui::UiPlugin;

mod world;
use crate::world::WorldPlugin;

mod save;

mod team;

mod creature;

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
        .add_plugins((LdtkPlugin, WorldPlugin, UiPlugin, CamPlugin, DexPlugin))
        // .add_plugins((CamPlugin, UiPlugin))
        .init_state::<AppState>()
        .run();
}
