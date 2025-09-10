use bevy::{
    app::Plugin,
    state::{app::AppExtStates, state::States},
};

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
    /// The selected action happens
    Action,
    /// Enemy turn
    EnemyTurn,
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
