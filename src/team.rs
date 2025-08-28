use bevy::ecs::resource::Resource;
use serde::{Deserialize, Serialize};

use crate::creature::Creature;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Team(pub Vec<TeamMember>);

impl Team {
    pub fn new() -> Self {
        Team(Vec::new())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub creature: Creature,
    pub hp: u8,
}

impl TeamMember {
    pub fn name(&self) -> &str {
        &self.creature.name
    }

    pub fn sprite(&self) -> String {
        self.creature.texture_path()
    }

    pub fn max_hp(&self) -> u8 {
        self.creature.stats.hp
    }
}
