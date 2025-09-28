use bevy::prelude::*;
use bevy_ecs_ldtk::{
    EntityInstance, GridCoords, LevelIid, LevelSelection, ldtk::FieldValue, prelude::LdtkFields,
    utils::grid_coords_to_translation,
};

use crate::{
    player::Player,
    utils::{Y_CHAR_OFFSET, read_dir_from_ldtk_entity},
    world::{
        GridSize,
        goals::components::{Goal, LevelGoals, WaitingTeleport},
    },
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
    player_q: Single<(Entity, &mut GridCoords), With<Player>>,
    goal_q: Query<(&EntityInstance, &GridCoords), (With<Goal>, Without<Player>)>,
) {
    let (entity, coords) = player_q.into_inner();
    if level_goals.goal_locations.contains(&*coords) {
        // only triggerred if the player walks on top of a goal
        for (entity_inst, g_coords) in goal_q {
            if *g_coords == *coords {
                // only run if player is walking on top of the goal
                let destination_value = entity_inst
                    .get_field_instance("destination")
                    .unwrap()
                    .value
                    .clone();
                let entry_entity_ref = match destination_value {
                    FieldValue::EntityRef(Some(x)) => x,
                    _ => panic!("Something aint right in {} metadata", entity_inst.iid),
                };
                let goal_direction = read_dir_from_ldtk_entity(entity_inst);
                let world_dest = entry_entity_ref.level_iid.clone();
                commands.insert_resource(LevelSelection::Iid(LevelIid::new(world_dest)));
                // Insert a [WaitingTeleport] so it can be processed later
                commands
                    .entity(entity)
                    .insert(WaitingTeleport(goal_direction, entry_entity_ref.entity_iid));
            }
        }
    }
}

pub fn handle_waiting_teleport(
    mut commands: Commands,
    waiting_q: Query<
        (Entity, &mut GridCoords, &mut Transform, &WaitingTeleport),
        With<WaitingTeleport>,
    >,
    goal_q: Query<(&EntityInstance, &GridCoords), (With<Goal>, Without<WaitingTeleport>)>,
    grid_size: Res<GridSize>,
) {
    for (entity, mut coords, mut trans, WaitingTeleport(dir, goal_iid)) in waiting_q {
        if let Some((_ei, gc)) = goal_q.iter().find(|(ei, _)| ei.iid == *goal_iid) {
            let new_coords = dir.next_coords(*gc);
            *coords = new_coords;
            trans.translation = grid_coords_to_translation(*coords, IVec2::splat(grid_size.0))
                .extend(trans.translation.z);
            trans.translation.y += Y_CHAR_OFFSET;
            // finally
            commands.entity(entity).remove::<WaitingTeleport>();
        } else {
            // do nothing, wait until the goals are loaded
        }
    }
}
