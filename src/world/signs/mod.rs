mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::{
    AppState,
    world::signs::{components::SignBundle, systems::*},
};

pub struct SignsPlugin;

impl Plugin for SignsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.register_ldtk_entity::<SignBundle>("Sign").add_systems(
            Update,
            (add_shiny_to_sign, handle_player_interaction_with_sign)
                .run_if(in_state(AppState::InGame)),
        );
    }
}
