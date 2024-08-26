use bevy::math::IVec2;
use bevy::prelude::{Changed, Query, Transform};
use bevy_ecs_ldtk::GridCoords;
use crate::constants::GRID_SIZE;

pub fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}