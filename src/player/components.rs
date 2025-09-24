use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};

use crate::animation::AnimationConfig;

#[derive(Default, PartialEq, Component, Clone, Debug)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
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
