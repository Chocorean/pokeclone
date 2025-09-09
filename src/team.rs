use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::index::{Creature, Dex};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Team(pub Vec<TeamMember>);

impl Team {
    pub fn new() -> Self {
        Team(Vec::new())
    }
}

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

    pub fn sprite(&self, dex: &Dex) -> String {
        dex.get_creature(self.creature_id).texture_path()
    }

    pub fn max_hp(&self, dex: &Dex) -> u8 {
        dex.get_creature(self.creature_id).stats.hp
    }
}
