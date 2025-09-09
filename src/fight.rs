use bevy::{
    app::Plugin,
    state::{app::AppExtStates, state::States},
};

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
    }
}

// ATTACKS RELATED STUFF

/// Target of an attack or item usage.
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

pub trait Attack {
    fn attack(&self);
}

struct PhysicalAttack {
    name: String,
    /// Required attributes to be able to use it.
    source: Vec<Attribute>,
    // x2
    pub strong_against: Vec<Attribute>,
    // x0.5
    pub weak_against: Vec<Attribute>,
    // no dmg
    pub useless_against: Vec<Attribute>,
    pub damage: Option<u8>,
    pub target_type: Target,
}

struct MagicAttack {
    name: String,
    element: Element,
}
