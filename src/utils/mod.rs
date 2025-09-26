mod ldtk;

use bevy::ecs::component::Component;
use bevy_ecs_ldtk::GridCoords;
pub use ldtk::*;

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

impl From<String> for Direction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "West" => Direction::Left,
            "East" => Direction::Right,
            "North" => Direction::Up,
            "South" => Direction::Down,
            x => panic!("unknown direction {x}"),
        }
    }
}
