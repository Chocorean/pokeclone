use bevy::prelude::*;

mod goals;
mod herbs;
mod npcs;
mod signs;
mod walls;

pub(crate) use herbs::LevelHerbs;
pub(crate) use npcs::{LevelNPCs, NPCKind};
pub(crate) use walls::LevelWalls;

use crate::{
    AppState,
    camera::WorldCamera,
    world::{
        goals::GoalsPlugin, herbs::HerbsPlugin, npcs::NPCsPlugin, signs::SignsPlugin,
        walls::WallsPlugin,
    },
};

#[derive(Resource)]
pub struct GridSize(pub i32);
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridSize(16))
            .add_plugins(GoalsPlugin)
            .add_plugins(HerbsPlugin)
            .add_plugins(NPCsPlugin)
            .add_plugins(SignsPlugin)
            .add_plugins(WallsPlugin)
            .add_systems(
                // When we leave the game
                // It's not an OnExit because we might leave this state when entering in combat or something
                OnTransition {
                    exited: AppState::InGame,
                    entered: AppState::MainMenu,
                },
                clean_up_world,
            );
    }
}

/// Despawn the world and its camera.
fn clean_up_world(mut commands: Commands, cam_q: Single<Entity, With<WorldCamera>>) {
    commands.entity(*cam_q).despawn();
}
