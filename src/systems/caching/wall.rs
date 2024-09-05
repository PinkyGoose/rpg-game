use std::collections::HashSet;
use bevy::log::info;
use crate::constants::GRID_SIZE;
use crate::entities::wall::{LevelWalls, Wall};
use bevy_ecs_ldtk::utils::translation_to_grid_coords;

use bevy::math::IVec2;
use bevy::prelude::{Changed, GlobalTransform, Query, ResMut,With};
use bevy_ecs_ldtk::GridCoords;

pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut query: Query<&GlobalTransform, (With<Wall>, Changed<GlobalTransform>)>,
) {
    let mut new_level_walls = HashSet::new();
    let vec_grid_size = IVec2::new(GRID_SIZE,GRID_SIZE);

            for global_transform in query.iter_mut() {

                let translation = global_transform.translation().truncate();
                info!("translation {:?}", translation);
                let mut grid_coords = translation_to_grid_coords(translation, vec_grid_size);//;-GridCoords::new(1,1);
                if grid_coords.x > 0{
                    grid_coords-=GridCoords::new(1,0);
                }
                if grid_coords.y > 0{
                    grid_coords-=GridCoords::new(0,1);

                }
                new_level_walls.insert(grid_coords);

            }
    if new_level_walls.is_empty(){
        return;
    }
    level_walls.wall_locations = new_level_walls;

}