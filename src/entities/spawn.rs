use crate::GRID_SIZE;
use bevy::{
    math::IVec2,
    prelude::{
        Bundle, Component, Deref, DerefMut, EventReader, Query, Reflect,
        ReflectResource, ResMut, Resource, Vec2, With,
    },
    utils::HashMap,
};
use bevy::log::info;
use bevy::prelude::{Transform, Without};
use bevy_ecs_ldtk::{
    prelude::LdtkFields,
    EntityIid, EntityInstance, GridCoords, LdtkSpriteSheetBundle, LevelEvent, LevelIid,
};
use bevy_ecs_ldtk_macros::LdtkEntity;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use crate::entities::player::Player;





pub fn spawn_player(
    mut level_events: EventReader<LevelEvent>,
    mut player: Query<&mut Transform, With<Player>>,
    spawn_points: Query<(&EntityIid, &Transform), (With<SpawnPoint>, Without<Player>)>,
    mut spawn_point: ResMut<SpawnPointId>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(_) = level_event {
            info!("начинаем переносить персонажа в другую локу");
            if let Some(point) = spawn_point.0.clone() {
                for (spawn, transform) in spawn_points.iter() {
                    if *spawn == point {
                        if let Ok(mut player_coords) = player.get_single_mut() {
                            player_coords.translation = transform.translation;
                        }
                    }
                }
                spawn_point.0 = None;
                return;
            }
        }
    }
}


#[derive(Debug, Default, Component, Reflect)]
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
pub struct UnresolvedIdRef {
    pub dest: Option<(EntityIid, LevelIid)>,
}



impl UnresolvedIdRef {
    pub fn from_entry_target_field(entity_instance: &EntityInstance) -> UnresolvedIdRef {
        // let k = entity_instance.get_entity_ref_field("mother");
        UnresolvedIdRef {
            dest: entity_instance
                .get_maybe_entity_ref_field("entry_target")
                .expect("expected entity to have mother entity ref field")
                .as_ref()
                .map(|entity_ref| {
                    (
                        EntityIid::new(entity_ref.entity_iid.clone()),
                        LevelIid::new(entity_ref.level_iid.clone()),
                    )
                }),
        }
    }
}
#[derive(Default, Component)]
pub struct EntryPoint;

#[derive(Default, Bundle, LdtkEntity)]
pub struct EntryPointBundle {
    goal: crate::entities::spawn::EntryPoint,
    #[grid_coords]
    grid_coords: GridCoords,
    #[with(UnresolvedIdRef::from_entry_target_field)]
    unresolved_mother: UnresolvedIdRef,
}

#[derive(Debug, Deref, DerefMut, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SpawnPointId(pub Option<EntityIid>);

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
