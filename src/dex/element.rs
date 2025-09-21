use std::fmt;

use serde::{Deserialize, Serialize};

/// There are 4 elements in the game. They have circular stregths/weaknesses, and slithgly
/// alter the stats of a creature.
#[derive(Copy, Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub enum Element {
    #[default]
    Fire,
    Air,
    Earth,
    Water,
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "fire" => Element::Fire,
            "air" => Element::Air,
            "earth" => Element::Earth,
            "water" => Element::Water,
            x => panic!("Unknown element type {x}"),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Air => "Air",
            Self::Fire => "Fire",
            Self::Earth => "Earth",
            Self::Water => "Water",
        })
    }
}
