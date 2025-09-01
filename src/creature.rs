use std::{fs::File, path::Path};

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
        app.init_resource::<Creature>(); // wild
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
        let mut species = Vec::new();
        for sp in json["species"]
            .as_array()
            .expect("species should be an array")
        {
            let mut creatures = Vec::new();
            for cr in sp["individuals"]
                .as_array()
                .expect("individuals should be an array")
            {
                creatures.push(Creature::from_value(cr));
            }
            species.push(Species::from_value(sp, creatures));
        }
        Dex { species: species }
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
}

#[derive(Clone)]
pub struct Species {
    name: String,
    mass: f32,   // kg
    height: f32, // m
    attributes: Vec<Attribute>,
    stats: Stats,
    individuals: Vec<Creature>,
}

impl Species {
    pub fn from_value(value: &Value, individuals: Vec<Creature>) -> Self {
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
            individuals,
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
}

impl Stats {
    // pub fn new(hp: u8, attack: u8, defense: u8, speed: u8) -> Self {
    //     Stats {
    //         hp,
    //         attack,
    //         defense,
    //         speed,
    //     }
    // }

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
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Resource, Default)]
pub struct Creature {
    pub name: String,
    // species: Species,
    pub stats: Stats,
    pub element: Element,
}

impl Creature {
    pub fn from_value(value: &Value) -> Self {
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
            _ => panic!("Unknown element type"),
        };
        // For simplicity, we use base stats as individual stats
        // In a real game, you would have variations
        let stats = Stats::from_value(value);
        Creature {
            name,
            stats,
            element,
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
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub enum Element {
    #[default]
    Fire,
    Air,
    Earth,
    Water,
}
