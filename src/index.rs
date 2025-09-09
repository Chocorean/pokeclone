use std::{fmt, path::Path};

use bevy::{
    app::{App, Plugin},
    ecs::resource::Resource,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct DexPlugin;

impl Plugin for DexPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dex::new());
        app.init_resource::<Creature>(); // wild encounter
    }
}

/// Hold all species and creatures in the game
/// Not a pokedex though, more like an encyclopedia (no discover mechanism)
#[derive(Resource)]
pub struct Dex {
    pub species: Vec<Species>,
}

impl Dex {
    pub fn new() -> Self {
        // Not using std::fs for WASM compatibility
        let content = include_str!("../assets/creatures/gen1.json");
        let json: serde_json::Value = serde_json::from_str(content).unwrap();
        let mut species_list: Vec<Species> = Vec::new();
        for sp in json["species"]
            .as_array()
            .expect("species should be an array")
        {
            let mut species = Species::from_value(sp);
            let mut creatures = Vec::new();
            for cr in sp["individuals"]
                .as_array()
                .expect("individuals should be an array")
            {
                creatures.push(Creature::from_value(cr, species_list.len(), &species.stats));
            }
            species.individuals = creatures;

            species_list.push(species);
        }
        Dex {
            species: species_list,
        }
    }

    pub fn individuals(&self) -> Vec<Creature> {
        self.species
            .iter()
            .map(|s| s.individuals.clone())
            .flatten()
            .collect()
    }

    pub fn random(&self) -> Creature {
        let individuals = self.individuals();
        let mut rng = rand::rng();
        let creature_idx = rng.random_range(0..individuals.len());
        individuals[creature_idx].clone()
    }

    pub fn get_creature(&self, ids: (usize, usize)) -> &Creature {
        self.species
            .get(ids.0)
            .unwrap()
            .individuals
            .get(ids.1)
            .unwrap()
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Species {
    pub name: String,
    mass: f32,   // kg
    height: f32, // m
    attributes: Vec<Attribute>,
    stats: Stats,
    pub individuals: Vec<Creature>,
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

/// Physical attributes that a creature can have
/// It determines physical attacks and damage multipliers
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
            x => panic!("Unknown attribute type {x}"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
/// Base stats of a species.
pub struct Stats {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    // common for all
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
}

impl IntoIterator for Stats {
    type Item = (String, u8);

    type IntoIter = StatsIntoIterator;

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
        let element_str = value["element"]
            .as_str()
            .expect("creature element should be a string");
        let element = match element_str {
            "Fire" => Element::Fire,
            "Air" => Element::Air,
            "Earth" => Element::Earth,
            "Water" => Element::Water,
            x => panic!("Unknown element type {x}"),
        };
        Creature {
            name,
            stats: Creature::compute_stats(&element, species_stats),
            element,
            species_id,
        }
    }

    pub fn texture_path(&self) -> String {
        let gif_assets_path = format!("assets/textures/creatures/{}.gif", self.name.to_lowercase());
        if Path::new(&gif_assets_path).exists() {
            format!("textures/creatures/{}.gif", self.name.to_lowercase())
        } else {
            format!("textures/creatures/{}.png", self.name.to_lowercase())
        }
    }

    fn compute_stats(element: &Element, species_stats: &Stats) -> Stats {
        let mut stats = species_stats.clone();
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

#[derive(Clone, Serialize, Deserialize, Default)]
pub enum Element {
    #[default]
    Fire,
    Air,
    Earth,
    Water,
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
