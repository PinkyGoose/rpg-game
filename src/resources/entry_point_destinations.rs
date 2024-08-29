use crate::GRID_SIZE;
use bevy::prelude::{Reflect, ReflectResource, Resource};
use bevy::math::{IVec2, Vec2};
use bevy::utils::HashMap;
use bevy_ecs_ldtk::{EntityIid, GridCoords, LevelIid};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
#[derive(Default, Reflect)]
pub struct Destination {
    pub level: LevelIid,
    pub spawn_point: EntityIid,
}
#[derive(Default, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct LevelEntryPoints {
    pub entry_point_destinations: HashMap<GridCoords, Destination>,
    pub level_width: i32,
    pub level_height: i32,
}

impl LevelEntryPoints {
    //TODO Нужна большая уборка
    pub fn in_entry_point(&self, grid_coords: &GridCoords) -> Option<&Destination> {
        if let Some(dest) = self.entry_point_destinations.get(grid_coords) {
            Some(dest)
        } else {
            None
        }
    }
    pub fn in_entry_point_with_size(&self, coords: &Vec2, size: i32) -> Option<&Destination> {
        let half_size = (size / 2 - 2) as f32;
        let minus_half_size = -1. * half_size;
        let coords = *coords;
        if let Some(dest) = self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
            coords + Vec2::new(half_size, half_size),
            IVec2::splat(GRID_SIZE),
        )) {
            Option::from(dest)
        } else if let Some(dest) =
            self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
                coords + Vec2::new(half_size, minus_half_size),
                IVec2::splat(GRID_SIZE),
            ))
        {
            Option::from(dest)
        } else if let Some(dest) =
            self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
                coords + Vec2::new(minus_half_size, half_size),
                IVec2::splat(GRID_SIZE),
            ))
        {
            Option::from(dest)
        } else if let Some(dest) =
            self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(
                coords + Vec2::new(minus_half_size, minus_half_size),
                IVec2::splat(GRID_SIZE),
            ))
        {
            Option::from(dest)
        } else {
            None
        }
    }
}
