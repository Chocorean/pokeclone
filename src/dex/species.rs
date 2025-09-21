use bevy::{asset::Handle, ecs::resource::Resource};
use bevy_easy_gif::prelude::GifAsset;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::element::Element;

/// Physical attributes that a creature can have
/// It determines physical attacks and damage multipliers?
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Attribute {
    Ears,
    Tail,
    Eyes,
    Wings,
    Paws,
    Teeth,
    Hair,
    Legs,
    Beak,
    Claws,
    Tongue,
}

impl Attribute {
    pub fn from_value(value: &Value) -> Self {
        let attr_str = value
            .as_str()
            .expect("attribute should be a string")
            .to_lowercase();
        match attr_str.as_str() {
            "ears" => Attribute::Ears,
            "tail" => Attribute::Tail,
            "eyes" => Attribute::Eyes,
            "wings" => Attribute::Wings,
            "paws" => Attribute::Paws,
            "teeth" => Attribute::Teeth,
            "hair" => Attribute::Hair,
            "legs" => Attribute::Legs,
            "beak" => Attribute::Beak,
            "claws" => Attribute::Claws,
            "tongue" => Attribute::Tongue,
            x => panic!("Unknown attribute type {x}"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
/// List of stats for a species or a creature. They act as "base stats", and are altered
/// in fight by active effects.
pub struct Stats {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    /// this guy is a %
    pub dodge: u8,
    /// this guy is a %
    pub accuracy: u8,
}

impl Stats {
    pub fn from_value(value: &Value) -> Self {
        let hp = value["stats"]["hp"]
            .as_u64()
            .expect("hp should be a positive integer") as u8;
        let attack = value["stats"]["attack"]
            .as_u64()
            .expect("attack should be a positive integer") as u8;
        let defense = value["stats"]["defense"]
            .as_u64()
            .expect("defense should be a positive integer") as u8;
        let speed = value["stats"]["speed"]
            .as_u64()
            .expect("speed should be a positive integer") as u8;
        Stats {
            hp,
            attack,
            defense,
            speed,
            // default for all
            dodge: 0,
            accuracy: 100,
        }
    }

    /// Slightly update stats based on an element (used for an individual)
    pub fn with_element(&self, element: Element) -> Self {
        let mut stats = self.clone();
        match element {
            Element::Fire => {
                stats.speed = (stats.speed as f32 * 1.05).round() as u8;
                stats.speed = (stats.defense as f32 * 0.95).round() as u8;
            }
            Element::Water => {
                stats.dodge += 5;
                stats.attack = (stats.defense as f32 * 0.95).round() as u8;
            }
            Element::Air => {
                stats.attack = (stats.speed as f32 * 1.05).round() as u8;
                stats.accuracy -= 5;
            }
            Element::Earth => {
                stats.defense = (stats.speed as f32 * 1.05).round() as u8;
                stats.speed = (stats.defense as f32 * 0.95).round() as u8;
            }
        }
        stats
    }
}

// Iterator implementation
impl IntoIterator for Stats {
    type Item = (String, u8);

    type IntoIter = StatsIntoIterator;

    /// This allows to loop over all stats seperately, useful to draw tables.
    fn into_iter(self) -> Self::IntoIter {
        StatsIntoIterator {
            stats: self,
            index: 0,
        }
    }
}

pub struct StatsIntoIterator {
    stats: Stats,
    index: usize,
}

impl Iterator for StatsIntoIterator {
    type Item = (String, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => (String::from("HP"), self.stats.hp),
            1 => (String::from("Attack"), self.stats.attack),
            2 => (String::from("Defense"), self.stats.defense),
            3 => (String::from("Speed"), self.stats.speed),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[derive(Clone, Serialize, Deserialize, Resource, Default)]
pub struct Creature {
    pub name: String,
    pub element: Element,
    pub species_id: usize,
    pub stats: Stats,
}

impl Creature {
    pub fn from_value(value: &Value, species_id: usize, species_stats: &Stats) -> Self {
        let name = value["name"]
            .as_str()
            .expect("creature name should be a string")
            .to_string();
        let element = value["element"]
            .as_str()
            .expect("creature element should be a string")
            .into();
        Creature {
            name,
            stats: species_stats.with_element(element),
            element,
            species_id,
        }
    }

    pub fn assets_texture_path(&self) -> String {
        format!("textures/creatures/{}.gif", self.name.to_lowercase())
    }

    pub fn texture_path(&self) -> String {
        format!("assets/{}", self.assets_texture_path())
    }
}

///
#[derive(Clone)]
#[allow(dead_code)]
pub struct Species {
    pub name: String,
    mass: f32,   // kg
    height: f32, // m
    pub attributes: Vec<Attribute>,
    pub stats: Stats,
    pub individuals: Vec<(Creature, Handle<GifAsset>)>,
}

impl Species {
    pub fn from_value(value: &Value) -> Self {
        let name = value["name"]
            .as_str()
            .expect("species name should be a string")
            .to_string();
        let mass = value["mass_kg"]
            .as_f64()
            .expect("mass_kg should be a number") as f32;
        let height = value["height_m"]
            .as_f64()
            .expect("height_m should be a number") as f32;
        let mut attributes = Vec::new();
        for attr in value["attributes"]
            .as_array()
            .expect("attributes should be an array")
        {
            let attribute = Attribute::from_value(attr);
            attributes.push(attribute);
        }
        let stats = Stats::from_value(value);
        Species {
            name,
            mass,
            height,
            attributes,
            stats,
            individuals: vec![],
        }
    }
}
