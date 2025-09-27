mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;
pub use systems::*;

use crate::AppState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (
                translate_grid_coords_entities,
                update_translation_after_save,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
