mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
pub(crate) use components::{LevelWalls, WallBundle};
use systems::cache_wall_locations;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<LevelWalls>()
            .register_ldtk_int_cell_for_layer::<WallBundle>("TileEntities", 1)
            .add_systems(Update, cache_wall_locations);
    }
}
