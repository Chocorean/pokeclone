use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords};

use crate::{
    player::Player,
    utils::{
        Direction, read_dir_from_ldtk_entity, read_npc_kind_from_ldtk_entity,
        read_str_from_ldtk_entity,
    },
    world::npcs::components::{LevelNPCs, NPC, NPCKind},
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
pub fn cache_npc_locations(
    mut level_walls: ResMut<LevelNPCs>,
    npcs: Query<(Entity, &GridCoords), Added<NPC>>,
    // moving_npcs: Query<(Entity, &GridCoords), Changed<MovingNPC>>,
) {
    for (npc, npc_coords) in npcs.iter() {
        level_walls.npcs_locations.insert(npc.index(), *npc_coords);
    }
}

pub fn add_sprite_to_npc(
    asset_server: ResMut<AssetServer>,
    npc_q: Query<(&mut Sprite, &EntityInstance), Added<NPC>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let librarian: Handle<Image> = asset_server.load("textures/npcs/librarian.png");
    let monk: Handle<Image> = asset_server.load("textures/npcs/monk.png");
    let writer: Handle<Image> = asset_server.load("textures/npcs/writer.png");

    for (mut sprite, entity) in npc_q {
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
    }
}
