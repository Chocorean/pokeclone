use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, utils::grid_coords_to_translation};

use crate::{
    utils::{SmoothMove, UpdatePosAfterSave, movement::Y_CHAR_OFFSET},
    world::GridSize,
};

pub fn update_translation_after_save(
    mut commands: Commands,
    entity_q: Query<(&mut Transform, &GridCoords, Entity), Added<UpdatePosAfterSave>>,
    grid_size: Res<GridSize>,
) {
    for (mut trans, coords, entity) in entity_q {
        trans.translation = grid_coords_to_translation(*coords, IVec2::splat(grid_size.0))
            .extend(trans.translation.z);
        trans.translation.y += Y_CHAR_OFFSET;
        commands.entity(entity).remove::<UpdatePosAfterSave>();
    }
}

/// Move everything on the map with [SmoothMove] accordingly to their [GridCoords]
pub fn translate_grid_coords_entities(
    mut commands: Commands,
    time: Res<Time>,
    grid_size: Res<GridSize>,
    mut grid_coords_entities: Query<
        (Entity, &mut Transform, &mut GridCoords, &mut SmoothMove),
        With<SmoothMove>,
    >,
) {
    for (entity, mut transform, mut grid_coords, mut smooth_move) in grid_coords_entities.iter_mut()
    {
        smooth_move.timer.tick(time.delta());

        let mut start_trans =
            grid_coords_to_translation(smooth_move.start, IVec2::splat(grid_size.0))
                .extend(transform.translation.z);
        start_trans.y += Y_CHAR_OFFSET;
        let mut end_trans = grid_coords_to_translation(smooth_move.end, IVec2::splat(grid_size.0))
            .extend(transform.translation.z);
        end_trans.y += Y_CHAR_OFFSET;

        let t = smooth_move.timer.fraction();
        transform.translation = start_trans.lerp(end_trans, t);

        if smooth_move.timer.finished() {
            transform.translation = end_trans;
            *grid_coords = smooth_move.end;

            commands.entity(entity).remove::<SmoothMove>();
        }
    }
}
