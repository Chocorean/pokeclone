use bevy::{
    asset::Handle,
    ecs::{resource::Resource, system::Res},
};
use bevy_easy_gif::GifAsset;
use serde::{Deserialize, Serialize};

use crate::dex::Dex;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct Team(pub Vec<TeamMember>);

impl Team {
    pub fn new() -> Self {
        Team(Vec::new())
    }

    /// Spawns a new team with `n` members (min 1, max 5)
    pub fn new_random(n: usize, dex: Res<Dex>) -> Self {
        let mut rng = rand::rng();
        let mut team = vec![];
        for _ in 0..n.max(1).min(5) {
            let (creature, _) = dex.random();
            let ids = dex.get_creature_ids(&creature);
            let hp = rand::Rng::random_range(&mut rng, 1..creature.stats.hp + 1);
            let member = TeamMember {
                surname: None,
                creature_id: (ids.0, ids.1),
                hp,
            };
            team.push(member);
        }
        Team(team)
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
            .unwrap_or(dex.get_creature(self.creature_id).0.name.clone())
    }

    pub fn max_hp(&self, dex: &Dex) -> u8 {
        dex.get_creature(self.creature_id).0.stats.hp
    }

    pub fn handle(&self, dex: &Dex) -> Handle<GifAsset> {
        dex.get_creature(self.creature_id).1.clone()
    }

    // todo base stat + actual stats (from combat)
}
