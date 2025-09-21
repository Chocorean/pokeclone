use std::sync::Arc;

use bevy::{
    app::{App, Plugin, PreStartup},
    asset::{AssetServer, Handle},
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
};
use bevy_easy_gif::prelude::GifAsset;
use rand::Rng;

use crate::{
    dex::attacks::{MagicalAttack, PhysicalAttack},
    team::TeamMember,
};

mod element;

mod species;
pub use species::{Creature, Species};

mod attacks;
pub use attacks::Attack;
use attacks::Attacks;

/// This plugin is responsible for loading all the data relevant to the creatures.
/// Namely sprites, attacks, elements, species, description...
/// Litterally anything to be shown in a complete index.
pub struct DexPlugin;

impl Plugin for DexPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_index);
        app.init_resource::<Creature>(); // wild encounter // todo to move else where or remove
    }
}

fn init_index(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(Dex::new(asset_server));
}

/// Hold all species and creatures in the game
/// Not a pokedex though, more like an encyclopedia (no discover mechanism)
#[derive(Resource)]
pub struct Dex {
    pub species: Vec<Species>,
    pub attacks: Attacks,
}

impl Dex {
    pub fn new(asset_server: ResMut<AssetServer>) -> Self {
        // loading creatures
        let content = include_str!("../../assets/creatures/gen1.json");
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
                let creature = Creature::from_value(cr, species_list.len(), &species.stats);
                let handle: Handle<GifAsset> = asset_server.load(creature.assets_texture_path());
                creatures.push((creature, handle));
            }
            species.individuals = creatures;

            species_list.push(species);
        }
        // attacks
        let content = include_str!("../../assets/creatures/attacks.json");
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
    pub fn individuals(&self) -> Vec<(Creature, Handle<GifAsset>)> {
        self.species
            .iter()
            .map(|s| s.individuals.clone())
            .flatten()
            .collect()
    }

    /// return a clone of a randomly selected creature
    pub fn random(&self) -> (Creature, Handle<GifAsset>) {
        let individuals = self.individuals();
        let mut rng = rand::rng();
        let creature_idx = rng.random_range(0..individuals.len());
        individuals[creature_idx].clone()
    }

    /// Query the specified creature
    /// could use newtypes here to reduce confusion.
    pub fn get_creature(&self, ids: (usize, usize)) -> &(Creature, Handle<GifAsset>) {
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
        self.filter_attacks_for_creature(creature.0.clone())
    }
}
