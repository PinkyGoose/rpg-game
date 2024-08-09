use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy::prelude::ReflectResource;
use bevy::prelude::{Added, Commands, Entity};
use bevy::prelude::Reflect;
use bevy_ecs_ldtk::{EntityInstance, LevelIid};
use crate::GRID_SIZE;
use std::collections::HashSet;
use bevy::log::info;
use bevy::math::{IVec2};
use bevy::utils::HashMap;
use bevy_ecs_ldtk::{EntityIid, LdtkSpriteSheetBundle};
use bevy_ecs_ldtk_macros::LdtkEntity;
use bevy_asset::{Assets, Handle};
use bevy_ecs_ldtk::assets::{LdtkProject, LevelMetadataAccessor};
use bevy::prelude::{Component, Deref, DerefMut, EventReader, Query, Res, ResMut, Resource, Vec2, With};
use bevy_ecs_ldtk::{GridCoords, LdtkIntCell, LevelEvent};
use bevy::prelude::Bundle;
use bevy_ecs_ldtk::ldtk::World;
use bevy_ecs_ldtk::prelude::LdtkFields;
#[derive(Debug,Default, Component, Reflect)]
pub struct SpawnPoint;

#[derive(Default, Bundle, LdtkEntity)]
pub struct SpawnPointBundle {
    goal: SpawnPoint,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Debug, Default, Deref, DerefMut, Component, Reflect)]
pub struct UnresolvedIdRef{
    pub dest: Option<(EntityIid, LevelIid)>
}
impl UnresolvedIdRef {
    pub fn from_mother_field(entity_instance: &EntityInstance) -> UnresolvedIdRef {
        // let k = entity_instance.get_entity_ref_field("mother");
        UnresolvedIdRef {

            dest: entity_instance
            .get_maybe_entity_ref_field("mother")
            .expect("expected entity to have mother entity ref field")
            .as_ref()
            .map( | entity_ref| (EntityIid::new(entity_ref.entity_iid.clone()), LevelIid::new(entity_ref.level_iid.clone()))),

        }
    }
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
    #[with(UnresolvedIdRef::from_mother_field)]
    unresolved_mother: UnresolvedIdRef,
}
pub fn cache_entry_point_locations(
    mut commands: Commands,
    // unresolved_mothers: Query<(Entity, &UnresolvedIdRef)>,
    mut level_entry_points: ResMut<LevelEntryPoints>,
    mut level_events: EventReader<LevelEvent>,
    entry_points: Query<(Entity, &GridCoords, &UnresolvedIdRef), (With<EntryPoint>, With<UnresolvedIdRef>)>,
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

            let mut entry_point_destinations= HashMap::new();
            for (id, coords,destination) in entry_points.iter(){
                if let Some(dest) = destination.as_ref().clone() {
                    entry_point_destinations.insert(*coords, Destination {
                        level: dest.1.clone(),
                        spawn_point: dest.0.clone()
                    });
                }
                        commands
                            .entity(id)
                            .remove::<UnresolvedIdRef>();
            }
            let new_level_entry_points = LevelEntryPoints {
                entry_point_destinations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_entry_points = new_level_entry_points;
        }
    }
}
#[derive(Debug, Deref, DerefMut, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SpawnPointId(pub Option<EntityIid>);

#[derive(Default, Reflect)]
pub struct Destination{
    pub level: LevelIid,
    pub spawn_point: EntityIid
}
#[derive(Default, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct LevelEntryPoints {
    // entry_point_locations: HashSet<GridCoords>,
    entry_point_destinations: HashMap<GridCoords, Destination>,
    level_width: i32,
    level_height: i32,
}

impl LevelEntryPoints {
    //TODO Нужна большая уборка
    pub fn in_entry_point(&self, grid_coords: &GridCoords) -> Option<&Destination> {
        if let Some(dest) = self.entry_point_destinations.get(grid_coords){
            Some(dest)
        }
        else { None }
    }
    pub fn in_entry_point_with_size(&self, coords: &Vec2, size: i32) -> Option<&Destination> {
        let half_size = (size/2-2) as f32;
        let minus_half_size = -1.*half_size;
        let coords = *coords;
        if let Some(dest) = self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(half_size, half_size), IVec2::splat(GRID_SIZE))){
            Option::from(dest)
        } else if let Some(dest) = self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(half_size, minus_half_size), IVec2::splat(GRID_SIZE))){
            Option::from(dest)
        } else if let Some(dest) = self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(minus_half_size, half_size), IVec2::splat(GRID_SIZE))){
            Option::from(dest)
        } else if let Some(dest) = self.in_entry_point(&bevy_ecs_ldtk::utils::translation_to_grid_coords(coords+Vec2::new(minus_half_size, minus_half_size), IVec2::splat(GRID_SIZE))) {
            Option::from(dest)
        } else { None }



    }

}