use bevy::prelude::*;
use bevy_ecs_ldtk::{
    EntityInstance, GridCoords, LevelIid, LevelSelection, ldtk::FieldValue, prelude::LdtkFields,
};

use crate::{
    player::Player,
    world::goals::components::{Goal, LevelGoals},
};

// todo seulement quand le niveau charge
pub fn cache_goal_locations(
    mut level_goals: ResMut<LevelGoals>,
    goals: Query<&GridCoords, With<Goal>>,
) {
    for goal_coords in goals.iter() {
        level_goals.goal_locations.insert(*goal_coords);
    }
}

pub fn handle_through_goal(
    mut commands: Commands,
    level_goals: ResMut<LevelGoals>,
    mut player_q: Single<&mut GridCoords, With<Player>>,
    goal_q: Query<(&EntityInstance, &GridCoords), (With<Goal>, Without<Player>)>,
    entry_q: Query<(&EntityInstance, &GridCoords), (With<Goal>, Without<Player>)>,
) {
    let coords = player_q.clone();
    if level_goals.goal_locations.contains(&coords) {
        // only triggerred if the player walks on top of a goal
        for (entity, g_coords) in goal_q {
            if *g_coords == **player_q {
                // only run if player is walking on top of the goal
                let destination_value = entity
                    .get_field_instance("destination")
                    .unwrap()
                    .value
                    .clone();
                let entry_entity_ref = match destination_value {
                    FieldValue::EntityRef(Some(x)) => x,
                    _ => panic!("Something aint right in {} metadata", entity.iid),
                };
                let world_dest = entry_entity_ref.level_iid.clone();
                // Does not work because the neighbors level are not loaded.
                let coords_dest = entry_q
                    .iter()
                    .find(|e| e.0.iid == entry_entity_ref.entity_iid)
                    .unwrap()
                    .1;
                commands.insert_resource(LevelSelection::Iid(LevelIid::new(world_dest)));
                **player_q = *coords_dest;
            }
        }
    }
}
