//! Renders a 2D scene containing a single, moving sprite.

use crate::systems::animation::{process_player, spawn_animations};
use crate::systems::caching::entry::cache_entry_point_locations;
use crate::systems::caching::wall::cache_wall_locations;
use crate::entities::spawn::spawn_player;
use crate::{
    constants::GRID_SIZE,
    entities::player::{Player},
    entities::spawn::{
        EntryPointBundle, LevelEntryPoints,
        SpawnPointId, UnresolvedIdRef,
    },
    entities::wall::{LevelWalls, WallBundle},
    movement::{move_all, move_player_from_input, randomize_movements},
};
use bevy::{
    DefaultPlugins,
    prelude::{
        App, Camera2dBundle, Changed, Commands, IVec2, PluginGroup, Query, Res, ResMut, Startup,
        Transform, Update, With,
    },
};
use bevy::prelude::IntoSystemConfigs;
use bevy_asset::AssetServer;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
    GridCoords, LdtkWorldBundle, LevelSelection,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_render::prelude::ImagePlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};
use entities::goat::GoatBundle;
use crate::entities::spawn::SpawnPointBundle;
mod constants;
mod movement;
mod entities;
mod systems;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_ecs_ldtk::LdtkPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::iid("bbd618c0-4ce0-11ef-9196-9768dcadd1bb"))
        // .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoatBundle>("Goat")
        .register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
        .register_ldtk_entity::<EntryPointBundle>("EntryPoint")
        .register_ldtk_int_cell::<WallBundle>(1)
        .init_resource::<LevelWalls>()
        .register_type::<SpawnPointId>()
        .register_type::<UnresolvedIdRef>()
        .init_resource::<LevelEntryPoints>()
        .insert_resource(SpawnPointId(None))
        .add_systems(
            Update,
            (
                move_player_from_input,
                translate_grid_coords_entities,
                cache_wall_locations,
                cache_entry_point_locations,
                move_all,
                randomize_movements,
                check_player_on_entry,
                process_player,
                spawn_player.after(process_player),
            ),
        )
        .add_systems(Startup,
            spawn_animations
        )
        .run();
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
                .extend(transform.translation.z);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);
    commands.spawn(PerfUiBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/firstmap.ldtk"),
        ..Default::default()
    });
}
fn check_player_on_entry(
    mut spawn_point_id: ResMut<SpawnPointId>,
    mut level_selection: ResMut<LevelSelection>,
    players: Query<&Transform, (With<Player>, Changed<Transform>)>,
    level_entries: Res<LevelEntryPoints>,
) {
    let mut dest = None;
    for player_grid_coords in players.iter() {
        if let Some(level) =
            level_entries.in_entry_point_with_size(&player_grid_coords.translation.truncate(), 16)
        {
            dest = Some(level);
            spawn_point_id.0 = Some(level.spawn_point.clone());
        }
    }
    if let Some(dest) = dest {
        *level_selection = LevelSelection::iid(dest.level.clone())
    }
}
