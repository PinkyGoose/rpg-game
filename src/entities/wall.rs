use crate::GRID_SIZE;
use bevy::log::debug;
use bevy::prelude::Bundle;
use bevy::prelude::Component;
use bevy::prelude::Resource;
use bevy::prelude::{IVec2, Vec2};
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_ecs_ldtk::GridCoords;
use bevy_ecs_ldtk::LdtkIntCell;
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
}

#[derive(Debug)]
pub enum DirectionY {
    Up,
    Down,
    None,
}
#[derive(Debug)]
pub enum DirectionX {
    Left,
    Right,
    None,
}
impl LevelWalls {
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        self.wall_locations.contains(grid_coords)
    }

    pub fn get_access_to_go(
        &self,
        coords_source: &Vec2,
        coords_dest: &Vec2,
        direction_x: DirectionX,
        direction_y: DirectionY,
    ) -> (DirectionX, DirectionY) {
        let grid_size = IVec2::new(GRID_SIZE, GRID_SIZE);
        let half_size = (GRID_SIZE / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = coords_dest.with_y(coords_source.y)
            + Vec2::new(
                match direction_x {
                    DirectionX::Left => minus_half_size,
                    DirectionX::Right => half_size,
                    DirectionX::None => 0.,
                },
                0.,
            );

        let player_grid_top =
            translation_to_grid_coords(coords + Vec2::new(0., half_size), grid_size);
        let player_grid_bottom =
            translation_to_grid_coords(coords + Vec2::new(0., minus_half_size), grid_size);

        let new_direction_x =
            match self.in_wall(&(player_grid_top)) || self.in_wall(&(player_grid_bottom)) {
                true => DirectionX::None,
                false => direction_x,
            };
        let coords = coords_dest.with_x(coords_source.x)
            + Vec2::new(
                0.,
                match direction_y {
                    DirectionY::Down => minus_half_size,
                    DirectionY::Up => half_size,
                    DirectionY::None => 0.,
                },
            );
        let player_grid_left =
            translation_to_grid_coords(coords + Vec2::new(minus_half_size, 0.), grid_size);
        let player_grid_right =
            translation_to_grid_coords(coords + Vec2::new(half_size, 0.), grid_size);

        let new_direction_y =
            match self.in_wall(&(player_grid_left)) || self.in_wall(&(player_grid_right)) {
                true => DirectionY::None,
                false => direction_y,
            };
        (new_direction_x, new_direction_y)
    }
    pub fn in_wall_horizontal_with_size(&self, coords: &Vec2, size: i32) -> bool {
        let half_size = (size / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = *coords;
        let player_grid = translation_to_grid_coords(
            coords + Vec2::new(0., half_size),
            IVec2::new(GRID_SIZE, GRID_SIZE),
        );
        self.in_wall(
            &(player_grid
                + GridCoords::new(
                    0,
                    match player_grid.y < 0 {
                        _ => 0,
                    },
                )),
        )
    }
    pub fn in_wall_vertical_with_size(&self, coords: &Vec2, size: i32) -> bool {
        // return false;
        let half_size = (size / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = *coords;
        let player_grid = translation_to_grid_coords(
            coords + Vec2::new(half_size, 0.),
            IVec2::new(GRID_SIZE, GRID_SIZE),
        );
        self.in_wall(&(player_grid))
    }
}
