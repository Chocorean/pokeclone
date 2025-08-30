use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;

use crate::{
    appstate::AppState,
    creature::Dex,
    world::{LevelHerbs, LevelWalls},
};

#[derive(Default, Component, Clone)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Default, Component)]
pub struct Player {}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    direction: Direction,
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
    mut commands: Commands,
    mut player_q: Query<(&mut GridCoords, &mut Direction), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
    level_herbs: Res<LevelHerbs>,
    mut next_state: ResMut<NextState<AppState>>,
    dex: Res<Dex>,
) {
    for (mut player_grid_coords, mut direction) in player_q.iter_mut() {
        // let (mut player_grid_coords, mut direction) = player_q.single_mut().unwrap();
        if input.just_pressed(KeyCode::KeyW) {
            *direction = Direction::Up;
        } else if input.just_pressed(KeyCode::KeyS) {
            *direction = Direction::Down;
        } else if input.just_pressed(KeyCode::KeyA) {
            *direction = Direction::Left;
        } else if input.just_pressed(KeyCode::KeyD) {
            *direction = Direction::Right;
        } else {
            return;
        };
        // direction.next_coords(*player_grid_coords);
        let destination = direction.next_coords(*player_grid_coords);
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
            if level_herbs.herb_locations.contains(&destination) {
                let mut rng = rand::rng();
                let nbr = rng.random::<u8>();
                if nbr < 64 {
                    // start a random encounter
                    let creature = dex.random();
                    commands.insert_resource(creature);
                    next_state.set(AppState::InFight);
                }
            }
        }
    }
}
