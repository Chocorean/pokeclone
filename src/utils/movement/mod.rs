mod components;
mod systems;

use bevy::prelude::*;

use bevy_ecs_ldtk::GridCoords;
pub use components::*;
pub use systems::*;

use crate::AppState;

pub const Y_CHAR_OFFSET: f32 = 4.;

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

pub trait GC {
    fn abs(&self) -> i32;
    fn next_step(&self, other: Self) -> Self;
}

impl GC for GridCoords {
    /// Return an approx of the norm of the vector [0;0] - `self`
    fn abs(&self) -> i32 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt() as i32
    }

    /// Give the next [GridCoords] when starting from `self` and
    /// going to `other`
    ///
    /// The direction goes from `self` to `other`, as a reminder
    fn next_step(&self, other: Self) -> Self {
        let mut offset = other - *self;
        let norm = offset.abs();
        offset.x /= norm;
        offset.y /= norm;
        let new = *self + offset;
        new
    }
}
