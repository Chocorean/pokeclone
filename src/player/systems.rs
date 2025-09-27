use bevy::prelude::*;
use bevy_easy_gif::{Gif, GifDespawn};
use bevy_ecs_ldtk::GridCoords;

use crate::{
    animation::{AnimationConfig, trigger_animation},
    event::MoveInBushEvent,
    utils::Direction,
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

pub fn move_player_from_input(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    grid_size: Res<GridSize>,
    player_q: Single<
        (
            &mut GridCoords,
            &mut Direction,
            &mut AnimationConfig,
            &mut Sprite,
        ),
        With<Player>,
    >,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
    level_npcs: Res<LevelNPCs>,
    level_herbs: Res<LevelHerbs>,
    mut event_writer: EventWriter<MoveInBushEvent>,
) {
    // Read keyboard input
    let (mut player_grid_coords, mut direction, mut animation, mut sprite) = player_q.into_inner();
    if input.just_pressed(KeyCode::KeyW) {
        *direction = Direction::Up;
        *animation = AnimationConfig::new(3, 5, 10);
    } else if input.just_pressed(KeyCode::KeyS) {
        *direction = Direction::Down;
        *animation = AnimationConfig::new(0, 2, 10);
    } else if input.just_pressed(KeyCode::KeyA) {
        *direction = Direction::Left;
        *animation = AnimationConfig::new(6, 8, 10);
    } else if input.just_pressed(KeyCode::KeyD) {
        *direction = Direction::Right;
        *animation = AnimationConfig::new(9, 11, 10);
    } else {
        return;
    };

    // Update coords and trigger other stuff
    let destination = direction.next_coords(*player_grid_coords);
    if !level_walls.in_wall(&destination) && !level_npcs.in_npc(&destination) {
        *player_grid_coords = destination;

        sprite.texture_atlas.as_mut().unwrap().index = animation.first_sprite_index;
        trigger_animation(&mut animation);

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
