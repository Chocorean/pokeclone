use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::condition::in_state,
};
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

mod components;
mod systems;

use components::NPCsBundle;

use crate::{AppState, utils::translate_grid_coords_entities, world::npcs::systems::*};
pub(crate) use components::{LevelNPCs, NPCKind};

pub struct NPCsPlugin;

impl Plugin for NPCsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<LevelNPCs>()
            .register_ldtk_entity::<NPCsBundle>("NPC")
            .register_ldtk_entity::<NPCsBundle>("MovingNPC")
            // both are registered under NPCsBundle
            .add_systems(
                Update,
                (
                    cache_npc_locations,
                    handle_player_interaction_with_npc,
                    add_sprite_to_npc,
                    update_moving_npc.before(translate_grid_coords_entities),
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(Update, init_moving_npcs);
    }
}
