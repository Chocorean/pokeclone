use bevy::prelude::*;
use bevy_easy_gif::{Gif, GifDespawn};
use bevy_ecs_ldtk::GridCoords;

use crate::{
    animation::{AnimationConfig, trigger_animation},
    event::MoveInBushEvent,
    utils::{Direction, SmoothMove},
    world::{GridSize, LevelHerbs, LevelNPCs, LevelWalls},
};

use super::components::Player;

/// Overwrite LTDK's atlas configuration.
pub fn setup_player_atlas(
    player_q: Single<&mut Sprite, Added<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut sprite = player_q.into_inner();
    if let Some(atlas) = &mut sprite.texture_atlas {
        // Configuring atlas layout
        let layout = TextureAtlasLayout::from_grid(UVec2::new(14, 21), 3, 4, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        atlas.layout = texture_atlas_layout;
    }
}

/// Logic for moving the [Player]
///
/// It wont run if it has the [SmoothMove] component,
/// which indicates the [Player] is still moving from previous
/// inputs.
pub fn move_player_from_input(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    grid_size: Res<GridSize>,
    player_q: Single<
        (
            Entity,
            &GridCoords,
            &mut Direction,
            &mut AnimationConfig,
            &mut Sprite,
        ),
        (With<Player>, Without<SmoothMove>),
    >,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
    level_npcs: Res<LevelNPCs>,
    level_herbs: Res<LevelHerbs>,
    mut event_writer: EventWriter<MoveInBushEvent>,
) {
    // Read keyboard input
    let (entity, player_grid_coords, mut direction, mut animation, mut sprite) =
        player_q.into_inner();
    if input.pressed(KeyCode::KeyW) {
        *direction = Direction::Up;
        *animation = AnimationConfig::new(3, 5, 10);
    } else if input.pressed(KeyCode::KeyS) {
        *direction = Direction::Down;
        *animation = AnimationConfig::new(0, 2, 10);
    } else if input.pressed(KeyCode::KeyA) {
        *direction = Direction::Left;
        *animation = AnimationConfig::new(6, 8, 10);
    } else if input.pressed(KeyCode::KeyD) {
        *direction = Direction::Right;
        *animation = AnimationConfig::new(9, 11, 10);
    } else {
        return;
    };

    let destination = direction.next_coords(*player_grid_coords);
    // trigger movement even if we hit a wall. makes it silly
    sprite.texture_atlas.as_mut().unwrap().index = animation.first_sprite_index;
    trigger_animation(&mut animation);

    if !level_walls.in_wall(&destination) && !level_npcs.in_npc(&destination) {
        commands
            .entity(entity)
            .insert(SmoothMove::new(*player_grid_coords, destination));

        dbg!("check for herb");
        dbg!(level_herbs.herb_locations.len());
        if level_herbs.herb_locations.contains(&destination) {
            event_writer.write(MoveInBushEvent);
        }
    } else {
        // ? gif on top of player
        let gif_pos = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *player_grid_coords + GridCoords::new(0, 1),
            IVec2::splat(grid_size.0),
        )
        .extend(50.); // high Z so it always show up
        let handle = asset_server.load("textures/animations/question.gif");
        commands.spawn((
            Gif { handle },
            Transform::from_translation(gif_pos),
            GifDespawn,
        ));
    }
}
