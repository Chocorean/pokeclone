use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LevelSelection, utils::grid_coords_to_translation};

use crate::{
    dex::Dex,
    player::Player,
    save::Save,
    utils::{UpdatePosAfterSave, Y_CHAR_OFFSET},
    world::GridSize,
};

/// Called by the UI after 'New Game' was pressed
///
/// initialize the team with one random creature
pub fn new_save(mut commands: Commands, dex: Res<Dex>) {
    let save = Save::default_with_team(1, dex);
    commands.insert_resource(LevelSelection::Identifier(save.level.clone()));
    commands.insert_resource(save);
}

/// Called by the UI after 'Resume' was pressed
pub fn load_save(mut commands: Commands) {
    // commands.insert_resource(save.team.clone()); // TODO should be moved elsewhere but it crashes the game

    let save = if Save::exists() {
        Save::load().unwrap()
    } else {
        Save::default()
    };
    commands.insert_resource(LevelSelection::Identifier(save.level.clone()));
    commands.insert_resource(save);
}

/// Apply the content of the save to the world.
/// It runs at most once because it removes the [Save] from the resources after loading it,
/// and does not run if it cannot find a [Save] [Resource].
pub fn apply_save(
    mut commands: Commands,
    player_q: Single<(Entity, &mut GridCoords, &mut Transform), With<Player>>,
    save: Res<Save>,
    grid_size: Res<GridSize>,
) {
    let (entity, mut player_coords, mut transform) = player_q.into_inner();
    *player_coords = GridCoords::new(save.coords.0, save.coords.1);
    transform.translation = grid_coords_to_translation(*player_coords, IVec2::splat(grid_size.0))
        .extend(transform.translation.z);
    transform.translation.y += Y_CHAR_OFFSET;
    commands.entity(entity).insert(UpdatePosAfterSave); // Do this for every entity that the save moves.
    // team
    commands.insert_resource(save.team.clone());
    // remove save
    commands.remove_resource::<Save>();
}
