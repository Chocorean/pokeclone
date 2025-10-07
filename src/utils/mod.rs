mod ldtk;
mod movement;

use bevy::ecs::component::Component;
use bevy_ecs_ldtk::GridCoords;

pub use ldtk::*;
pub use movement::*;

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

    /// This assume `one` and `two` are on the same row, or column, in that order.
    /// Then it returns the direction one would be facing if standing on `one`, looking at `two`
    pub fn from_coords(one: GridCoords, two: GridCoords) -> Self {
        let diff = two - one;
        if diff.y == 0 && one.x < two.x {
            Direction::Right
        } else if diff.y == 0 {
            Direction::Left
        } else if diff.x == 0 && one.y < two.y {
            Direction::Down
        } else {
            Direction::Up
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
