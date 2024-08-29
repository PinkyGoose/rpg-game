use crate::resources::entry_point_destinations::Destination;
use crate::resources::entry_point_destinations::LevelEntryPoints;
use bevy::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, With};
use bevy::utils::HashMap;
use bevy_asset::{Assets, Handle};
use bevy_ecs_ldtk::{GridCoords, LevelEvent};
use bevy_ecs_ldtk::assets::LdtkProject;
use crate::constants::GRID_SIZE;
use crate::entities::spawn::{EntryPoint, UnresolvedIdRef};
use bevy_ecs_ldtk::assets::LevelMetadataAccessor;
pub fn cache_entry_point_locations(
    mut commands: Commands,
    // unresolved_mothers: Query<(Entity, &UnresolvedIdRef)>,
    mut level_entry_points: ResMut<LevelEntryPoints>,
    mut level_events: EventReader<LevelEvent>,
    entry_points: Query<
        (Entity, &GridCoords, &UnresolvedIdRef),
        (With<EntryPoint>, With<UnresolvedIdRef>),
    >,
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

            let mut entry_point_destinations = HashMap::new();
            for (id, coords, destination) in entry_points.iter() {
                if let Some(dest) = destination.as_ref().clone() {
                    entry_point_destinations.insert(
                        *coords,
                        Destination {
                            level: dest.1.clone(),
                            spawn_point: dest.0.clone(),
                        },
                    );
                }
                commands.entity(id).remove::<UnresolvedIdRef>();
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