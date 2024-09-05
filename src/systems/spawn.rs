use bevy::math::{IVec2, UVec2};
use bevy::prelude::Vec3;
use bevy::prelude::{
    default, Changed, Commands, Entity, Query, Res, ResMut, SpriteBundle, TextureAtlas,
    TextureAtlasLayout, Transform, With,
};
use bevy::prelude::{Component, DespawnRecursiveExt, GlobalTransform, Resource};
use bevy::utils::{HashMap, HashSet};
use bevy_asset::{AssetServer, Assets};
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_ecs_ldtk::{GridCoords, LevelIid, LevelSelection};
use bevy_spritesheet_animation::component::SpritesheetAnimation;
use bevy_spritesheet_animation::library::SpritesheetLibrary;
use num::range_inclusive;

use crate::entities::health::{Health, Regeneration};
use crate::entities::level_params::LevelCoords;
use crate::entities::level_params::LevelSizes;
use crate::entities::player::{Player, PlayerBundle};
use crate::GRID_SIZE;

#[derive(Resource)]
pub struct PlayerSpawnPosition {
    pub x: f32,
    pub y: f32,
}
pub fn process_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<&Player>,
    player_spawn: Res<PlayerSpawnPosition>, // Используем ресурс с координатами для спавна
) {
    // Проверяем, есть ли уже заспавненный игрок
    if !player_query.is_empty() {
        return;
    }

    // Загружаем текстуру и создаем атлас
    let texture = assets.load("archer.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        3,
        3,
        None,
        None,
    ));

    // Спавним игрока с координатами из ресурса PlayerSpawnPosition
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(player_spawn.x, player_spawn.y, 3.0)
                .with_scale(Vec3::new(0.5, 0.5, 0.5)), // Используем координаты из ресурса

            ..default()
        },
        TextureAtlas {
            layout,
            ..default()
        },
        SpritesheetAnimation::from_id(library.animation_with_name("archer_idle").unwrap()),
        PlayerBundle { ..default() },
        Health {
            current: 34.0, // Изначальное значение здоровья
            max: 100.0,
        },
        Regeneration(8.0),
    ));
}

#[derive(Default, Debug, Resource)]
pub struct MyLevelNeighbors {
    pub jija: HashMap<GridCoords, LevelIid>,
}

#[derive(Default, Debug, Component)]
pub struct NeedToCacheNeighbors;

pub fn cache_neighbor_levels(
    mut commands: Commands,
    need_to_cache_neighbors: Query<Entity, With<NeedToCacheNeighbors>>,
    level_selection: Res<LevelSelection>,
    levels: Query<&LevelIid, With<LevelIid>>,
    mut my_level_neighbors: ResMut<MyLevelNeighbors>,
    level_sizes: Res<LevelSizes>,
    level_coords: Res<LevelCoords>,
) {
    if need_to_cache_neighbors.is_empty() {
        return;
    }
    for need in need_to_cache_neighbors.iter() {
        commands.entity(need).despawn_recursive();
    }
    // let mut k = HashMap::new();
    let mut vec_neighbors = Vec::new();

    let current_level = if let LevelSelection::Iid(id) = level_selection.as_ref() {
        for k in levels.iter() {
            if id != k {
                vec_neighbors.push(k);
            }
        }
        id
    } else {
        return;
    };

    let current_level_size = if let Some(some) = level_sizes.sizes.get(current_level) {
        some
    } else {
        return;
    };
    let current_level_pos = if let Some(some) = level_coords.sizes.get(current_level) {
        some
    } else {
        return;
    };

    let mut neighbor_grid_map = HashMap::new();

    let current_level_left_bottom = current_level_pos.grid_coords - GridCoords::new(1, 1);
    let current_level_right_top =
        current_level_pos.grid_coords + GridCoords::from(current_level_size.grid_size);
    // info!("current_level_left_bottom {:?} {:?}",current_level, current_level_left_bottom);
    // info!("current_level_right_top {:?}", current_level_right_top);
    let mut hash_set_current_level = HashSet::new();
    for i in range_inclusive(current_level_left_bottom.x, current_level_right_top.x) {
        hash_set_current_level.insert(GridCoords::new(i, current_level_right_top.y));
        hash_set_current_level.insert(GridCoords::new(i, current_level_left_bottom.y));
    }
    for i in range_inclusive(current_level_left_bottom.y, current_level_right_top.y) {
        hash_set_current_level.insert(GridCoords::new(current_level_right_top.x, i));
        hash_set_current_level.insert(GridCoords::new(current_level_left_bottom.x, i));
    }
    // info!("for current_level exits {:?} {:?}",hash_set_current_level.clone().into_iter().count(), hash_set_current_level);

    for level_neighbor in vec_neighbors {
        let level_neighbor_size = if let Some(some) = level_sizes.sizes.get(level_neighbor) {
            some
        } else {
            return;
        };
        let level_neighbor_pos = if let Some(some) = level_coords.sizes.get(level_neighbor) {
            some
        } else {
            return;
        };
        let level_neighbor_left_bottom = level_neighbor_pos.grid_coords;
        let level_neighbor_right_top = level_neighbor_pos.grid_coords
            + GridCoords::from(level_neighbor_size.grid_size)
            - GridCoords::new(1, 1);
        // info!("level_neighbor_left_bottom {:?} {:?}",level_neighbor, level_neighbor_left_bottom);
        // info!("level_neighbor_right_top {:?}", level_neighbor_right_top);
        let mut hash_set_level_neighbor = HashSet::new();
        for i in range_inclusive(level_neighbor_left_bottom.x, level_neighbor_right_top.x) {
            hash_set_level_neighbor.insert(GridCoords::new(i, level_neighbor_right_top.y));
            hash_set_level_neighbor.insert(GridCoords::new(i, level_neighbor_left_bottom.y));
            // info!("Идем по иксу {:?} {:?} {:?}",i,level_neighbor_right_top.y,level_neighbor_left_bottom.y)
        }
        for i in range_inclusive(level_neighbor_left_bottom.y, level_neighbor_right_top.y) {
            hash_set_level_neighbor.insert(GridCoords::new(level_neighbor_right_top.x, i));
            hash_set_level_neighbor.insert(GridCoords::new(level_neighbor_left_bottom.x, i));
        }
        for neighbor_grid in &hash_set_level_neighbor {
            if let Some(a) = hash_set_current_level.take(neighbor_grid) {
                neighbor_grid_map.insert(a, level_neighbor.clone());
            }
        }
        // info!("for level_neighbor exits {:?} {:?}",hash_set_level_neighbor.clone().into_iter().count(), hash_set_level_neighbor);
    }

    my_level_neighbors.jija = neighbor_grid_map;
}

pub fn check_player_on_entry(
    mut level_selection: ResMut<LevelSelection>,
    players: Query<&GlobalTransform, (With<Player>, Changed<Transform>)>,
    coords_exits: Res<MyLevelNeighbors>,
) {
    if let Ok(player) = players.get_single() {
        if let Some(a) = coords_exits.jija.get(&translation_to_grid_coords(
            player.translation().truncate(),
            IVec2::new(GRID_SIZE, GRID_SIZE),
        )) {
            // info!("level to move {:?}", a);
            *level_selection = LevelSelection::iid(a.clone())
        }
    }
}
