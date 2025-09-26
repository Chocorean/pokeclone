use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};

use crate::{animation::AnimationConfig, utils::Direction};

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    direction: Direction,
    animation: AnimationConfig,
}
