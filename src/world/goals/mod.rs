mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::{
    AppState,
    world::goals::{
        components::{GoalBundle, LevelGoals},
        systems::*,
    },
};

pub struct GoalsPlugin;

impl Plugin for GoalsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<LevelGoals>()
            // TODO Change Walls in ldtk and reflect here + walls
            .register_ldtk_entity::<GoalBundle>("Goal")
            .add_systems(
                Update,
                (handle_through_goal, cache_goal_locations).run_if(in_state(AppState::InGame)),
            );
    }
}
