use std::time::Duration;

use bevy::{
    ecs::component::Component,
    time::{Timer, TimerMode},
};
use bevy_ecs_ldtk::GridCoords;

/// This component should be insert for every entity after a [Save] changes its location.
/// It will be removed later after the translation of the entity will be updated according
/// to its [GridCoords].
#[derive(Component)]
pub struct UpdatePosAfterSave;

#[derive(Component, Debug)]
pub struct SmoothMove {
    pub start: GridCoords,
    pub end: GridCoords,
    pub timer: Timer,
}

impl SmoothMove {
    pub fn new(start: GridCoords, end: GridCoords) -> Self {
        Self {
            start,
            end,
            timer: Timer::new(Duration::from_millis(250), TimerMode::Once),
        }
    }
}
