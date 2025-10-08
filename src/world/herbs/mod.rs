mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
pub use components::{HerbBundle, LevelHerbs};
pub use systems::cache_herb_locations;

use crate::{AppState, world::herbs::systems::*};

pub struct HerbsPlugin;

impl Plugin for HerbsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<LevelHerbs>()
            .register_ldtk_int_cell_for_layer::<HerbBundle>("TileEntities", 2)
            .add_systems(Update, cache_herb_locations)
            .add_systems(Update, walk_in_herbs.run_if(in_state(AppState::InGame)));
    }
}
