use std::sync::Arc;

use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use super::species::Attribute;
use super::{element::Element, species::Species};

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
pub(crate) struct PhysicalAttack {
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
pub(crate) struct MagicalAttack {
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
    fn damage(&self) -> Option<u8>;
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

    fn damage(&self) -> Option<u8> {
        self.damage
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

    fn damage(&self) -> Option<u8> {
        self.damage
    }
}

/// Wrapper for storing all attacks
#[derive(Resource, Clone)]
pub struct Attacks(pub(crate) Vec<Arc<dyn Attack + Send + Sync>>);

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
