use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::index::Dex;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Team(pub Vec<TeamMember>);

impl Team {
    pub fn new() -> Self {
        Team(Vec::new())
    }
}

/// A team member is different from a creature. it shares a lot with a creature, but
/// at the end of the day it has an additional state which is its current health points.
/// It should also have a list of active effects for the fight, and eventually cross fight effects like statuses.
#[derive(Clone, Serialize, Deserialize)]
pub struct TeamMember {
    surname: Option<String>,
    pub creature_id: (usize, usize),
    pub hp: u8,
}

impl TeamMember {
    pub fn name(&self, dex: &Dex) -> String {
        self.surname
            .clone()
            .unwrap_or(dex.get_creature(self.creature_id).name.clone())
    }

    pub fn texture_path(&self, dex: &Dex) -> String {
        dex.get_creature(self.creature_id).texture_path()
    }

    pub fn max_hp(&self, dex: &Dex) -> u8 {
        dex.get_creature(self.creature_id).stats.hp
    }

    // todo base stat + actual stats (from combat)
}
