mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
pub use components::{HerbBundle, LevelHerbs};
use systems::cache_herb_locations;

use crate::AppState;

pub struct HerbsPlugin;

impl Plugin for HerbsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<LevelHerbs>()
            // TODO Change Walls in ldtk and reflect here + walls
            .register_ldtk_int_cell_for_layer::<HerbBundle>("Walls", 2)
            .add_systems(
                Update,
                cache_herb_locations.run_if(in_state(AppState::InGame)),
            );
    }
}
