use crate::GRID_SIZE;
use std::collections::HashSet;
use bevy::log::info;
use bevy::math::{IVec2};
use bevy::utils::HashMap;
use bevy_ecs_ldtk:: LdtkSpriteSheetBundle;
use bevy_ecs_ldtk_macros::LdtkEntity;
use bevy_asset::{Assets, Handle};
use bevy_ecs_ldtk::assets::{LdtkProject, LevelMetadataAccessor};
use bevy::prelude::{Component, EventReader, Query, Res, ResMut, Resource, Vec2, With};
use bevy_ecs_ldtk::{GridCoords, LdtkIntCell, LevelEvent};
use bevy::prelude::Bundle;

#[derive(Default, Component)]
pub struct SpawnPoint;

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpawnPointBundle {
    goal: SpawnPoint,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}



#[derive(Default, Component)]
pub struct EntryPoint;

#[derive(Default, Bundle, LdtkEntity)]
pub struct EntryPointBundle {
    goal: crate::spawn::EntryPoint,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
pub fn cache_entry_point_locations(
    mut level_entry_points: ResMut<LevelEntryPoints>,
    mut level_events: EventReader<LevelEvent>,
    entry_points: Query<&GridCoords, With<EntryPoint>>,
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

            let entry_point_locations = entry_points.iter().copied().collect();

            let new_level_entry_points = LevelEntryPoints {
                entry_point_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_entry_points = new_level_entry_points;
        }
    }
}

#[derive(Default, Resource)]
pub struct LevelEntryPoints {
    entry_point_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelEntryPoints {
    //TODO разделить на проверки по вертикали и по горизонтали
    pub fn in_entry_point(&self, grid_coords: &GridCoords) -> bool {
        info!("SIZE {:?}", self.entry_point_locations.len());
        grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.entry_point_locations.contains(grid_coords)
    }
    pub fn in_entry_point_with_size(&self, coords: &Vec2, size: i32) -> bool {
        let half_size = (size/2-2) as f32;
        let minus_half_size = -1.*half_size;
        let coords = *coords;
        // info!("POSITION {:?}",coords+Vec2::new(minus_half_size, minus_half_size));
        self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(half_size, half_size), IVec2::splat(GRID_SIZE)))||
            self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(half_size, minus_half_size), IVec2::splat(GRID_SIZE)))||
            self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(minus_half_size, half_size), IVec2::splat(GRID_SIZE)))||
            self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(minus_half_size, minus_half_size), IVec2::splat(GRID_SIZE)))||
            coords.x+minus_half_size<0.||
            coords.y+minus_half_size<0.

    }

}