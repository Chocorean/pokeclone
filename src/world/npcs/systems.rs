use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LevelEvent};

use crate::{
    player::Player,
    utils::{
        Direction, GC, SmoothMove, Y_CHAR_OFFSET, read_dir_from_ldtk_entity,
        read_direction_from_ldtk_entity, read_npc_kind_from_ldtk_entity, read_str_from_ldtk_entity,
    },
    world::npcs::components::{LevelNPCs, MovingNPCSchedule, NPC, NPCKind},
};

/// Handle for players interacting with NPC
/// Might need some refactoring around reading the json values
pub fn handle_player_interaction_with_npc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&GridCoords, &Direction), With<Player>>,
    npc_q: Query<(&GridCoords, &EntityInstance), With<NPC>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        let (player_grid_coords, direction) = player_q.single().unwrap();
        let facing_coords = direction.next_coords(*player_grid_coords);
        for (npc_coords, npc) in npc_q.iter() {
            if npc_coords == &facing_coords {
                // Access custom fields by name
                let chat = read_str_from_ldtk_entity("chat", npc);
                println!("NPC says: {}", chat);
            }
        }
    }
}

// Cache static NPCs locations.
// todo update pos of moving npcs.
pub fn cache_npc_locations(
    mut level_walls: ResMut<LevelNPCs>,
    npcs: Query<(&GridCoords, Option<&SmoothMove>), With<NPC>>,
) {
    level_walls.npcs_locations = HashSet::new();

    for (npc_coords, sm) in npcs.iter() {
        match sm {
            None => {
                level_walls.npcs_locations.insert(*npc_coords);
            }
            Some(sm) => {
                level_walls.npcs_locations.insert(sm.start);
                level_walls.npcs_locations.insert(sm.end);
            }
        };
    }
}

pub fn add_sprite_to_npc(
    asset_server: ResMut<AssetServer>,
    npc_q: Query<(&mut Sprite, &mut Transform, &EntityInstance), Added<NPC>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let librarian: Handle<Image> = asset_server.load("textures/npcs/librarian.png");
    let monk: Handle<Image> = asset_server.load("textures/npcs/monk.png");
    let writer: Handle<Image> = asset_server.load("textures/npcs/writer.png");

    for (mut sprite, mut transform, entity) in npc_q {
        let direction = read_dir_from_ldtk_entity(entity);
        sprite.image = match read_npc_kind_from_ldtk_entity(entity) {
            NPCKind::Librarian => librarian.clone(),
            NPCKind::Monk => monk.clone(),
            NPCKind::Writer => writer.clone(),
        };
        let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 20), 9, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let atlas = TextureAtlas {
            layout: texture_atlas_layout,
            index: match direction {
                Direction::Down => 0,
                Direction::Up => 1,
                _ => 2,
            },
        };
        sprite.texture_atlas = Some(atlas);
        sprite.flip_x = direction == Direction::Right;

        transform.translation.y += Y_CHAR_OFFSET;
    }
}

/// Run once when joining the world. The purpose of this system is to
/// make moving NPCs easily findable, but inserting a "next destination" component.
///
/// Runs once per level load.
pub(crate) fn init_moving_npcs(
    mut level_events: EventReader<LevelEvent>,
    mut commands: Commands,
    npcs: Query<(Entity, &EntityInstance), With<NPC>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Transformed(_) = level_event {
            for (entity, ldtk_entity) in npcs {
                if let Some(path) = read_direction_from_ldtk_entity(ldtk_entity) {
                    commands.entity(entity).insert(MovingNPCSchedule {
                        next: (*path.last().unwrap()).next_step(*path.first().unwrap()),
                        current: 0,
                        path,
                    });
                }
            }
        }
    }
}

/// Handle the movement of the NPCs, update direction and sprites
pub(crate) fn update_moving_npc(
    mut commands: Commands,
    npcs: Query<
        (
            Entity,
            &GridCoords,
            &mut Sprite,
            &mut MovingNPCSchedule,
            Option<&SmoothMove>,
        ),
        With<MovingNPCSchedule>,
    >,
    player: Single<(Option<&SmoothMove>, &GridCoords), With<Player>>,
) {
    for (entity, &gc, mut sprite, mut schedule, smooth_move) in npcs {
        if smooth_move.is_some() {
            // we want to wait until an NPC has finished moving before updating it again
            continue;
        }
        let copy = schedule.clone();
        let dest = schedule.update();
        // first we cancel the move if the player is in the way
        let player_cells = if let Some(sm) = player.0 {
            [sm.start, sm.end].to_vec()
        } else {
            [*player.1].to_vec()
        };
        if player_cells.contains(&dest) {
            *schedule = copy;
            continue;
        }
        // eventually, we move
        let sm = SmoothMove::new(gc, dest);
        commands.entity(entity).insert(sm);
        // sprite
        let dir = Direction::from_coords(gc, dest);
        // not set already, fine
        if sprite.texture_atlas.is_none() {
            continue;
        }
        let mut atlas = sprite.texture_atlas.clone().unwrap();
        match dir {
            Direction::Up => atlas.index = 0,
            Direction::Down => atlas.index = 1,
            _ => atlas.index = 2,
        }
        sprite.texture_atlas = Some(atlas);
        sprite.flip_x = matches!(dir, Direction::Right);
    }
}
