use bevy::{
    app::Plugin,
    ecs::resource::Resource,
    state::{app::AppExtStates, state::States},
};
use serde::{Deserialize, Serialize};

use crate::index::{Attribute, Element};

/// Describe the state of a fight.
#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub enum FightState {
    /// Setting up the UI and shit maybe
    Start,
    #[default]
    /// Player chooses if they want to attack, use item, switch, or flee.
    MainAction,
    /// Player chooses which of their creature will do or receive the following things.
    SourceChoice,
    /// Player chooses which attack they will use
    AttackChoice,
    /// Player chooses the item to use on their guys
    ItemChoice,
    /// Player chooses the target of the attack or item
    TargetChoice,
    /// Player chooses which creature they will send to replace the current one
    SwitchChoice,
    /// Give XP or level up or shit
    Win,
    /// Send back to last bed or smth.
    Lose,
}

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<FightState>();
        app.insert_resource(Attacks(build_attacks()));
    }
}

fn build_attacks() -> Vec<Box<dyn Attack + Send + Sync>> {
    let mut list: Vec<Box<dyn Attack + Send + Sync>> = vec![];

    let content: &'static str = include_str!("../assets/attacks.json");
    let json: serde_json::Value = serde_json::from_str(content).unwrap();
    for pa in json["physical_attacks"]
        .as_array()
        .expect("phys atks should be an array")
    {
        list.push(Box::new(PhysicalAttack::from_value(pa)));
    }
    for ma in json["magical_attacks"]
        .as_array()
        .expect("magic atks should be an array")
    {
        list.push(Box::new(MagicalAttack::from_value(ma)));
    }
    println!("attacks: {}", list.len());
    list
}

#[derive(Resource)]
struct Attacks(Vec<Box<dyn Attack + Send + Sync>>);

// ATTACKS RELATED STUFF

/// Target of an attack or item usage.
#[derive(Serialize, Deserialize)]
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
    AllBytSelf,
}

// Eventually implement effects
// pub enum Effect {
//     StatusChange(Status),
//     StatsChange(&str, f32, usize),
// }

pub trait Attack {
    fn attack(&self);
}

#[derive(Serialize, Deserialize)]
struct PhysicalAttack {
    pub name: String,
    /// Required attributes to be able to use it.
    pub source: Vec<Attribute>,
    // x2
    pub strong_against: Vec<Attribute>,
    // x0.5
    pub weak_against: Vec<Attribute>,
    // no dmg
    pub useless_against: Vec<Attribute>,
    pub target_type: Target,
    pub damage: Option<u8>,
    // pub effect: Option<Effect>,
}

impl PhysicalAttack {
    // todo: load for real
    pub fn from_value(value: &serde_json::Value) -> Self {
        Self {
            name: value.get("name").expect("should have a name").to_string(),
            source: value
                .get("source")
                .expect("should have source")
                .as_array()
                .unwrap()
                .iter()
                .map(|attr| Attribute::from_value(attr))
                .collect::<Vec<Attribute>>(),
            strong_against: vec![],
            weak_against: vec![],
            useless_against: vec![],
            target_type: Target::All,
            damage: if let Some(v) = value.get("damage") {
                Some(v.as_u64().unwrap() as u8)
            } else {
                None
            },
        }
    }
}

impl Attack for PhysicalAttack {
    fn attack(&self) {}
}

struct MagicalAttack {
    pub name: String,
    pub element: Element,
    pub damage: Option<u8>,
}

impl MagicalAttack {
    pub fn from_value(value: &serde_json::Value) -> Self {
        MagicalAttack {
            name: value.get("name").expect("should have a name").to_string(),
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

impl Attack for MagicalAttack {
    fn attack(&self) {}
}
