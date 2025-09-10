use std::{fmt, path::Path, sync::Arc};

use bevy::{
    app::{App, Plugin},
    ecs::resource::Resource,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::team::TeamMember;

/// This plugin is responsible for loading all the data relevant to the creatures.
/// Namely sprites, attacks, elements, species, description...
/// Litterally anything to be shown in a complete index.
pub struct DexPlugin;

impl Plugin for DexPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dex::new());
        app.init_resource::<Creature>(); // wild encounter // todo to move else where or remove
    }
}

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

///
#[derive(Clone)]
#[allow(dead_code)]
pub struct Species {
    pub name: String,
    mass: f32,   // kg
    height: f32, // m
    pub attributes: Vec<Attribute>,
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

/// Target of an attack or item usage.
#[derive(Serialize, Deserialize, Clone)]
pub enum Target {
    /// One enemy only
    Enemy,
    /// All enemies
    Enemies,
    /// All allies
    Allies,
    /// One ally only
    Ally,
    /// Everyone
    All,
    /// Self only
    OneSelf,
    /// Just can harm itself
    AllButSelf,
}

impl From<&str> for Target {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "enemy" => Self::Enemy,
            "enemies" => Self::Enemies,
            "ally" => Self::Ally,
            "allies" => Self::Allies,
            "all" => Self::All,
            "self" | "oneself" => Self::OneSelf,
            "allbutself" | "abs" => Self::AllButSelf,
            x => panic!("unknown attribute {x}"),
        }
    }
}

/// In-fight effect altering status (frozen, burnt..) or stats
#[derive(Serialize, Deserialize, Clone)]
pub enum Effect {
    // StatusChange(Status),
    // StatsChange(&str, f32, usize),
    // ...
}

/// Attacks derived from physical `Attribute`s
#[derive(Serialize, Deserialize, Clone)]
struct PhysicalAttack {
    pub name: String,
    /// Required attributes to be able to use it.
    pub source: Vec<Attribute>,
    // x2
    pub strong_against: Option<Vec<Attribute>>,
    // x0.5
    pub weak_against: Option<Vec<Attribute>>,
    // no dmg
    pub useless_against: Option<Vec<Attribute>>,
    pub target_type: Target,
    pub damage: Option<u8>,
    pub effects: Option<Vec<Effect>>,
}

impl PhysicalAttack {
    // todo: load for real
    pub fn from_value(value: &serde_json::Value) -> Self {
        Self {
            name: value
                .get("name")
                .expect("should have a name")
                .as_str()
                .unwrap()
                .to_string(),
            source: value
                .get("source")
                .expect("should have source")
                .as_array()
                .unwrap()
                .iter()
                .map(|attr| Attribute::from_value(attr))
                .collect::<Vec<Attribute>>(),
            strong_against: None,
            weak_against: None,
            useless_against: None,
            target_type: value.get("target").unwrap().as_str().unwrap().into(),
            damage: if let Some(v) = value.get("damage") {
                Some(v.as_u64().unwrap() as u8)
            } else {
                None
            },
            effects: None,
        }
    }
}

/// Each creature gets a magical attack based on their element.
#[derive(Clone)]
struct MagicalAttack {
    pub name: String,
    pub element: Element,
    pub damage: Option<u8>,
}

impl MagicalAttack {
    pub fn from_value(value: &serde_json::Value) -> Self {
        MagicalAttack {
            name: value
                .get("name")
                .expect("should have a name")
                .as_str()
                .unwrap()
                .to_string(),
            element: value["element"]
                .as_str()
                .expect("creature element should be a string")
                .into(),
            damage: if let Some(v) = value.get("damage") {
                Some(v.as_u64().unwrap() as u8)
            } else {
                None
            },
        }
    }
}

/// Common trait for physical and magical attacks
pub trait Attack {
    fn name(&self) -> String;
    fn attributes(&self) -> Vec<Attribute>;
    fn element(&self) -> Option<Element>;
}

impl Attack for PhysicalAttack {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn attributes(&self) -> Vec<Attribute> {
        self.source.clone()
    }

    fn element(&self) -> Option<Element> {
        None
    }
}

impl Attack for MagicalAttack {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn attributes(&self) -> Vec<Attribute> {
        vec![]
    }

    fn element(&self) -> Option<Element> {
        Some(self.element)
    }
}

/// Wrapper for storing all attacks
#[derive(Resource, Clone)]
pub struct Attacks(Vec<Arc<dyn Attack + Send + Sync>>);

impl Attacks {
    /// Filter all compatible attacks for a given creature.
    pub fn filter_for_species(&self, species: &Species) -> Self {
        let attacks = self
            .0
            .iter()
            .filter(|atk| {
                atk.element().is_none()
                    && atk
                        .attributes()
                        .iter()
                        .all(|attr| species.attributes.contains(attr))
            })
            .map(|arc| Arc::clone(arc))
            .collect();
        Attacks(attacks)
    }

    pub fn filter_by_elem(&self, element: Element) -> Self {
        let attacks = self
            .0
            .iter()
            .filter(|atk| {
                if let Some(elt) = atk.element() {
                    dbg!(elt, element, elt == element);
                    elt == element
                } else {
                    false
                }
            })
            .map(|arc| Arc::clone(arc))
            .collect();
        Attacks(attacks)
    }
}

pub struct AttacksIntoIterator {
    attacks: Attacks,
    index: usize,
}

impl IntoIterator for Attacks {
    type Item = Arc<dyn Attack + Send + Sync>;

    type IntoIter = AttacksIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        AttacksIntoIterator {
            attacks: self,
            index: 0,
        }
    }
}

impl Iterator for AttacksIntoIterator {
    type Item = Arc<dyn Attack + Send + Sync>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if let Some(arc) = self.attacks.0.get(self.index) {
            let arc = Arc::clone(arc);
            Some(arc)
        } else {
            None
        };

        self.index += 1;
        result
    }
}

/// Hold all species and creatures in the game
/// Not a pokedex though, more like an encyclopedia (no discover mechanism)
#[derive(Resource)]
pub struct Dex {
    pub species: Vec<Species>,
    pub attacks: Attacks,
}

impl Dex {
    pub fn new() -> Self {
        // loading creatures
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
        // attacks
        let content = include_str!("../assets/creatures/attacks.json");
        let json: serde_json::Value = serde_json::from_str(content).unwrap();
        let mut attacks: Attacks = Attacks(vec![]);
        for pa in json["physical_attacks"]
            .as_array()
            .expect("phys atks should be an array")
        {
            attacks.0.push(Arc::new(PhysicalAttack::from_value(pa)));
        }
        for ma in json["magical_attacks"]
            .as_array()
            .expect("magic atks should be an array")
        {
            attacks.0.push(Arc::new(MagicalAttack::from_value(ma)));
        }
        Dex {
            species: species_list,
            attacks,
        }
    }

    /// return a copy of all creatures
    pub fn individuals(&self) -> Vec<Creature> {
        self.species
            .iter()
            .map(|s| s.individuals.clone())
            .flatten()
            .collect()
    }

    /// return a clone of a randomly selected creature
    pub fn random(&self) -> Creature {
        let individuals = self.individuals();
        let mut rng = rand::rng();
        let creature_idx = rng.random_range(0..individuals.len());
        individuals[creature_idx].clone()
    }

    /// Query the specified creature
    /// could use newtypes here to reduce confusion.
    pub fn get_creature(&self, ids: (usize, usize)) -> &Creature {
        self.species
            .get(ids.0)
            .unwrap()
            .individuals
            .get(ids.1)
            .unwrap()
    }

    /// Return all compatible attacks for a given creature
    pub fn filter_attacks_for_creature(&self, creature: Creature) -> Attacks {
        let species = self.species.get(creature.species_id).unwrap();
        let mut attacks = self.attacks.filter_for_species(species);
        // jsut 4 elemental attacks for now, so just pick the one.
        attacks
            .0
            .extend(self.attacks.filter_by_elem(creature.element).0);
        attacks
    }

    /// Return all compatible attacks for a given team member
    pub fn filter_attacks_for_team_member(&self, member: TeamMember) -> Attacks {
        let creature = self.get_creature(member.creature_id);
        self.filter_attacks_for_creature(creature.clone())
    }
}
