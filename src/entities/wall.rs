use std::collections::HashSet;
use bevy::prelude::Resource;
use bevy::prelude::Bundle;
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_ldtk::LdtkIntCell;
use bevy::prelude::Component;
use crate::GRID_SIZE;
use bevy::prelude::{Vec2,IVec2};
#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
#[derive(Component, Debug)]
pub struct Walls;
#[derive(Default, Resource)]
pub struct LevelWalls {
    pub wall_locations: HashSet<GridCoords>,
}
impl LevelWalls {
    //TODO разделить на проверки по вертикали и по горизонтали
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
            self.wall_locations.contains(grid_coords)
    }
    pub fn in_wall_horizontal_with_size(&self, coords: &Vec2, size: i32) -> bool {
        // return false;
        let half_size = (size / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = *coords;
        self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(0., half_size),
            IVec2::splat(GRID_SIZE),
        )) || self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(0., minus_half_size),
            IVec2::splat(GRID_SIZE),
        ))
            //|| coords.y + minus_half_size < 0.
    }
    pub fn in_wall_vertical_with_size(&self, coords: &Vec2, size: i32) -> bool {
        // return false;
        let half_size = (size / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = *coords;
        self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(half_size, 0.),
            IVec2::splat(GRID_SIZE),
        )) || self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(minus_half_size, 0.),
            IVec2::splat(GRID_SIZE),
        ))
            //|| coords.x + minus_half_size < 0.
    }

}
