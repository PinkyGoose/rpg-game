
use bevy::prelude::info;
use crate::constants::GRID_SIZE;
use bevy_ecs_ldtk::prelude::LdtkProject;
use std::collections::HashSet;
use bevy::asset::{Assets, Handle};
use bevy::log::debug;
use bevy::math::{IVec2, Vec3};
use bevy::prelude::{Bundle, Component, EventReader, Query, Res, ResMut, Resource, Vec2, With};
use bevy_ecs_ldtk::{GridCoords, LdtkIntCell, LevelEvent};
use bevy_ecs_ldtk::assets::LevelMetadataAccessor;


pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_events: EventReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}
#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
#[derive(Default, Resource)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    //TODO разделить на проверки по вертикали и по горизонтали
    pub fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
    pub fn in_wall_with_size(&self, coords: &Vec2, size: i32) -> bool {
        let half_size = (size/2-2) as f32;
        let minus_half_size = (-1.*half_size);
        let coords = *coords;
        self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(half_size, half_size), IVec2::splat(GRID_SIZE)))||
            self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(half_size, minus_half_size), IVec2::splat(GRID_SIZE)))||
            self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(minus_half_size, half_size), IVec2::splat(GRID_SIZE)))||
            self.in_wall(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(minus_half_size, minus_half_size), IVec2::splat(GRID_SIZE)))||
            coords.x+minus_half_size<0.||
            coords.y+minus_half_size<0.

    }

}