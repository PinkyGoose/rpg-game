use bevy::hierarchy::BuildChildren;
use bevy::math::{IVec2, UVec2, Vec2, Vec3};
use bevy::prelude::{Component, DespawnRecursiveExt, GlobalTransform, info, Resource};
use bevy::prelude::{Added, Changed, Commands, default, Entity, Query, Res, ResMut, SpriteBundle, TextureAtlas, TextureAtlasLayout, Transform, With};
use bevy::utils::{HashMap, HashSet};
use bevy_asset::{Assets, AssetServer};
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LevelIid, LevelSelection, LevelSet};
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_spritesheet_animation::component::SpritesheetAnimation;
use bevy_spritesheet_animation::library::SpritesheetLibrary;
use num::{range_inclusive, range_step};

use crate::entities::health::{Health, Regeneration};
use crate::entities::player::{Player, PlayerBundle, PlayerChild};
use crate::GRID_SIZE;
use crate::systems::caching::entry::LevelSizes;

pub fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance, &Transform), Added<EntityInstance>>,
    library: ResMut<SpritesheetLibrary>,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player: Query<&Player>
)
{
    if let Some((entity, _, transform)) = new_entity_instances.iter().find(|(_, s2, _)| {
        if s2.identifier == "Player".to_string() {
            return true;
        }
        false
    }) {
        //TODO спавнить игрока самостоятельно
        if !player.is_empty(){
            return;
        }
        // if entity_instance.identifier == "Player".to_string() {
            info!("переписываем игрока");

        let texture = assets.load("archer.png");

        let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),
            3,
            3,
            None,
            None,
        ));
        commands
            // .entity(entity)
            // .insert(Player)
            .spawn((SpriteBundle {
                texture,
                transform: Transform::from_xyz(transform.translation.x,transform.translation.y,100.),
                ..default()
            }, TextureAtlas {
                layout,
                ..default()
            }, SpritesheetAnimation::from_id(library.animation_with_name("archer_idle").unwrap()),
                     PlayerBundle {
                         ..default()
                     },
                     Health {
                         current: 34.0, // Изначальное значение здоровья
                         max: 100.0,
                     },
                     Regeneration(8.),

            ))
            .with_children(|commands| {
                commands.spawn(PlayerChild);
            });
    }
    // }
}

#[derive(Default, Debug, Resource)]
pub struct MyLevelNeighbors {
    pub jija: HashMap<GridCoords, LevelIid>,
}

#[derive(Default, Debug, Component)]
pub struct NeedToCacheNeighbors;

pub fn cache_neighbor_levels(
    mut commands: Commands,
    need_to_cache_neighbors: Query<Entity,With<NeedToCacheNeighbors>>,
    player_global_coords: Query<&GlobalTransform, With<Player>>,
    level_selection: Res<LevelSelection>,
    // level_set: Query<&LevelSet, Changed<LevelSet>>,
    // ldtk_project_entities: Query<&Handle<LdtkProject>>,
    // ldtk_project_assets: Res<Assets<LdtkProject>>,
    levels: Query<(&Transform, &GlobalTransform, &LevelIid), With<LevelIid>>,
    mut my_level_neighbors: ResMut<MyLevelNeighbors>,
    level_sizes: Res<LevelSizes>,
) {
    if need_to_cache_neighbors.is_empty(){
        return;
    }
    for need in need_to_cache_neighbors.iter(){
        // commands.
        commands.entity(need).despawn_recursive();
        // info!("ДЕСПАВНИМ МАРКЕР");
    }
    let mut k = HashMap::new();
    let mut vec_neighbors = Vec::new();
    let mut level_current = None;
    if let LevelSelection::Iid(id) = level_selection.as_ref() {
        for (i, j, k) in levels.iter() {
            if id != k {
                vec_neighbors.push((i, j, k));
            } else {
                level_current = Some((i, j, k));
            }
        }
    }

    // info!("LEVELS {:?}", levels);
    // info!("I DALSHE {:?}", vec_neighbors);

    if let Some((transform, global_transform, id)) = level_current {
        let current_level_size = if let Some(some) = level_sizes.sizes.get(id) {
            some
        } else { return; };
        let transforms_left = global_transform.translation().truncate()
            // - Vec2::new(current_level_size.0 as f32, current_level_size.1 as f32) / 2.
            - Vec2::new(1., 1.);
        let transforms_right = global_transform.translation().truncate()
            + Vec2::new(current_level_size.0 as f32, current_level_size.1 as f32)
            + Vec2::new(1., 1.);
        let mut hash_set_grids = HashSet::new();
        let grid_size_vec = IVec2::new(GRID_SIZE,GRID_SIZE);
        // info!("level_size {:?}", current_level_size);
        // info!("global, left, right \n{:?}\n{:?}\n{:?}",global_transform, transforms_left, transforms_right);
        for i in range_inclusive(0, current_level_size.0 / GRID_SIZE) {
            let vec_to_coords = Vec2::new(transforms_left.x + (GRID_SIZE*i) as f32,transforms_left.y);
            hash_set_grids.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
            let vec_to_coords = Vec2::new(transforms_right.x - (GRID_SIZE*i) as f32,transforms_right.y);
            hash_set_grids.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
        }
        for i in range_inclusive(0, current_level_size.1 / GRID_SIZE) {
            let vec_to_coords = Vec2::new(transforms_left.x ,transforms_left.y+ (GRID_SIZE*i) as f32);
            hash_set_grids.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
            let vec_to_coords = Vec2::new(transforms_right.x ,transforms_right.y- (GRID_SIZE*i) as f32);
            hash_set_grids.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
        }
        // info!("gridcoords :{:?}", hash_set_grids);
        //+Vec2::new(current_level_size.0 as f32, current_level_size.1 as f32)
        for ((n_transform, n_global_transform, n_id)) in vec_neighbors {
            let translation = n_transform.translation;
            let level_size = if let Some(some) = level_sizes.sizes.get(n_id) {
                some
            } else { return; };
            let mut grids_of_neighbor = HashSet::new();
            let n_transforms_left = n_global_transform.translation().truncate()
                // - Vec2::new(current_level_size.0 as f32, current_level_size.1 as f32) / 2.
                + Vec2::new(1., 1.);
            let n_transforms_right = n_global_transform.translation().truncate()
                + Vec2::new(level_size.0 as f32, level_size.1 as f32)
                - Vec2::new(1., 1.);
            for i in range_inclusive(0, level_size.0 / GRID_SIZE) {
                let vec_to_coords = Vec2::new(n_transforms_left.x + (GRID_SIZE*i) as f32,n_transforms_left.y);
                grids_of_neighbor.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
                let vec_to_coords = Vec2::new(n_transforms_right.x - (GRID_SIZE*i) as f32,n_transforms_right.y);
                grids_of_neighbor.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
            }
            for i in range_inclusive(0, level_size.1 / GRID_SIZE) {
                let vec_to_coords = Vec2::new(n_transforms_left.x ,n_transforms_left.y+ (GRID_SIZE*i) as f32);
                grids_of_neighbor.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
                let vec_to_coords = Vec2::new(n_transforms_right.x ,n_transforms_right.y- (GRID_SIZE*i) as f32);
                grids_of_neighbor.insert(translation_to_grid_coords(vec_to_coords,grid_size_vec));
            }
            // info!("N level_size {:?}", level_size);
            // info!("N grid_coords :{:?}", grids_of_neighbor);
            // info!("N global, left, right \n{:?}\n{:?}\n{:?}",n_global_transform, n_transforms_left, n_transforms_right);
            for neighbor_grid in &grids_of_neighbor {
                if let Some(a) = hash_set_grids.take(neighbor_grid) {
                    k.insert(a, n_id.clone());
                }
            }
            // for i in hash_set_grids{
            //     k.insert(i, format!("{:?}", i));
            // }
        }

    }
    // info!("Neigbors {:?}", k);
    // if let Ok(level_set) = &level_set.get_single() {
    //     for i in &level_set.iids {
    //         if let LevelSelection::Iid(id) = level_selection.as_ref() {
    //             if id != i {
    //                 let level = ldtk_project
    //                     .get_raw_level_by_iid(i.get())
    //                     .expect("spawned level should exist in project");
    //                 // player
    //             }
    //         }
    //     }
    // }
    my_level_neighbors.jija = k;
    // info!("level_set {:?} level_selection {:?}",k, r)
}


pub fn check_player_on_entry(
    // mut spawn_point_id: ResMut<SpawnPointId>,
    mut level_selection: ResMut<LevelSelection>,
    players: Query<&GlobalTransform, (With<Player>, Changed<Transform>)>,
    // level_entries: Res<LevelEntryPoints>,
    level_set: Query<&LevelSet>,
    coords_exits: Res<MyLevelNeighbors>
) {
    if let Ok(player) = players.get_single(){

        if let Some(a) = coords_exits.jija.get( &translation_to_grid_coords(player.translation().truncate(), IVec2::new(GRID_SIZE,GRID_SIZE))){
            info!("level to move {:?}", a);
            *level_selection = LevelSelection::iid(a.clone())
        }
        // if player.
    }
    // let mut dest = None;
    // if let Ok(player_coords) = players.get_single() {
    //     if player_coords.translation.x < 0. {
    //         for i in &level_set.get_single().unwrap().iids {
    //             // if *level_selection != i{
    //             //
    //             // }
    //             // info!("sosedi {:?}", i);
    //             match level_selection.clone() {
    //                 LevelSelection::Identifier(_) => {}
    //                 LevelSelection::Indices(_) => {}
    //                 LevelSelection::Iid(id) => {
    //                     if id != i.clone() {
    //                         *level_selection = LevelSelection::iid(i.clone())
    //                     }
    //                 }
    //                 LevelSelection::Uid(_) => {}
    //             }
    //         }
    //     }
    //     // if let Some(level) =
    //     //     level_entries.in_entry_point_with_size(&player_grid_coords.translation.truncate(), 16)
    //     // {
    //     //     dest = Some(level);
    //     //     spawn_point_id.0 = Some(level.spawn_point.clone());
    //     // }
    // }
    // if let Some(dest) = dest {
    //     *level_selection = LevelSelection::iid(dest.level.clone())
    // }
}