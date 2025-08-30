use bevy::ecs::resource::Resource;
use bevy_ecs_ldtk::GridCoords;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::team::Team;

const SAVE_PATH: &str = "assets/saves/save.json";

#[derive(Serialize, Deserialize, Resource)]
pub struct Save {
    pub level: i32,
    pub coords: (i32, i32),
    pub team: Team,
}

impl Save {
    pub fn new(level: i32, coords: GridCoords, team: Team) {
        let save = Save {
            level,
            coords: (coords.x, coords.y),
            team,
        };
        save.write()
    }

    pub fn write(&self) {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        self.serialize(&mut ser).unwrap();
        // println!("{}", String::from_utf8(buf).unwrap());
        fs::write(SAVE_PATH, buf).unwrap();
    }

    pub fn exists() -> bool {
        fs::exists(SAVE_PATH).unwrap_or(false)
    }

    pub fn load() -> Option<Save> {
        let content = fs::read_to_string(SAVE_PATH).ok()?;
        let save: Save = serde_json::from_str(&content).unwrap();
        Some(save)
    }
}
