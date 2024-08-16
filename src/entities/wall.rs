
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_ldtk::LdtkIntCell;
use crate::GRID_SIZE;
use bevy::{
    math::IVec2,
    prelude::{Bundle, Component, Resource, Vec2},
};

use std::collections::HashSet;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
#[derive(Default, Resource)]
pub struct LevelWalls {
    pub wall_locations: HashSet<GridCoords>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelWalls {
    //TODO разделить на проверки по вертикали и по горизонтали
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
    pub fn in_wall_with_size(&self, coords: &Vec2, size: i32) -> bool {
        let half_size = (size / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = *coords;
        self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(half_size, half_size),
            IVec2::splat(GRID_SIZE),
        )) || self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(half_size, minus_half_size),
            IVec2::splat(GRID_SIZE),
        )) || self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(minus_half_size, half_size),
            IVec2::splat(GRID_SIZE),
        )) || self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(minus_half_size, minus_half_size),
            IVec2::splat(GRID_SIZE),
        )) || coords.x + minus_half_size < 0.
            || coords.y + minus_half_size < 0.
    }
}
