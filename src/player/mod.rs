use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::{
    AppState,
    player::{
        components::PlayerBundle,
        systems::{move_player_from_input, setup_player_atlas},
    },
};

mod components;
mod systems;

pub use components::{Direction, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player");
        app.add_systems(
            Update,
            (move_player_from_input, setup_player_atlas).run_if(in_state(AppState::InGame)),
        );
    }
}
