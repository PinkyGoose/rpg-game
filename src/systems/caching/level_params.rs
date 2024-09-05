use bevy::math::IVec2;
use bevy::prelude::{Commands, EventReader, GlobalTransform, Query, Res, ResMut};
use bevy_asset::{Assets, Handle};
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_ecs_ldtk::assets::LevelMetadataAccessor;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_ecs_ldtk::{LevelEvent, LevelIid};

use crate::constants::GRID_SIZE;
use crate::entities::level_params::LevelCoord;
use crate::entities::level_params::LevelCoords;
use crate::entities::level_params::LevelSize;
use crate::entities::level_params::LevelSizes;
use crate::systems::spawn::NeedToCacheNeighbors;

pub fn cache_level_params(
    mut commands: Commands,
    mut level_events: EventReader<LevelEvent>,
    levels: Query<(&GlobalTransform, &LevelIid)>,
    mut level_coords: ResMut<LevelCoords>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_sizes: ResMut<LevelSizes>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");
            for (transform, id) in levels.iter() {
                if id == level_iid {
                    let translation = transform.translation().truncate();
                    level_coords.sizes.insert(
                        level_iid.clone(),
                        LevelCoord {
                            grid_coords: translation_to_grid_coords(
                                translation,
                                IVec2::new(GRID_SIZE, GRID_SIZE),
                            ),
                            pix_coords: translation,
                        },
                    );
                }
            }

            level_sizes.sizes.insert(
                level_iid.clone(),
                LevelSize {
                    grid_size: IVec2::new(level.px_wid / GRID_SIZE, level.px_hei / GRID_SIZE),

                    pix_size: IVec2::new(level.px_wid, level.px_hei),
                },
            );
        }
        commands.spawn(NeedToCacheNeighbors);
    }
}
