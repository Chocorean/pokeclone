use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    AppState,
    animation::{AnimationConfig, trigger_animation},
    event::MoveInBushEvent,
    world::{LevelHerbs, LevelWalls},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup_player_atlas.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Default, PartialEq, Component, Clone, Debug)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    direction: Direction,
    animation: AnimationConfig,
}

/// Overwrite LTDK's atlas configuration.
fn setup_player_atlas(
    player_q: Single<&mut Sprite, With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut has_run: Local<bool>,
) {
    if *has_run {
        return;
    }

    let mut sprite = player_q.into_inner();
    if let Some(atlas) = &mut sprite.texture_atlas {
        // Configuring atlas layout
        let layout = TextureAtlasLayout::from_grid(UVec2::new(14, 21), 3, 4, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        atlas.layout = texture_atlas_layout;
    }

    // Don't run again!
    *has_run = true;
}

impl Direction {
    /// Return coords of the first cell in the facing direction
    pub fn next_coords(&self, coords: GridCoords) -> GridCoords {
        match self {
            Direction::Up => GridCoords::new(coords.x, coords.y + 1),
            Direction::Down => GridCoords::new(coords.x, coords.y - 1),
            Direction::Left => GridCoords::new(coords.x - 1, coords.y),
            Direction::Right => GridCoords::new(coords.x + 1, coords.y),
        }
    }
}

pub fn move_player_from_input(
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

    sprite.texture_atlas.as_mut().unwrap().index = animation.first_sprite_index;
    trigger_animation(&mut animation);

    // Update coords and trigger other stuff
    let destination = direction.next_coords(*player_grid_coords);
    if !level_walls.in_wall(&destination) {
        *player_grid_coords = destination;
        if level_herbs.herb_locations.contains(&destination) {
            event_writer.write(MoveInBushEvent {});
        }
    }
}
