//! Renders a 2D scene containing a single, moving sprite.

use crate::spawn::{cache_entry_point_locations, EntryPoint, EntryPointBundle, LevelEntryPoints, SpawnPointBundle};
use crate::spawn::SpawnPoint;
use crate::goat::GoatBundle;
use crate::player::{Player, PlayerBundle};
use crate::constants::GRID_SIZE;
use bevy_ecs_ldtk::LdtkSpriteSheetBundle;
use bevy::prelude::{KeyCode, ResMut};
use bevy::prelude::ButtonInput;
use bevy::prelude::With;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy::prelude::Camera2dBundle;
use bevy::prelude::Res;
use bevy::prelude::Commands;
use bevy::prelude::Changed;
use bevy_ecs_ldtk::GridCoords;
use bevy::prelude::Transform;
use bevy::prelude::Query;
use crate::wall::cache_wall_locations;
use bevy::prelude::Update;
use crate::wall::LevelWalls;
use crate::wall::WallBundle;
use bevy_ecs_ldtk::LevelSelection;
use bevy::prelude::Startup;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::DefaultPlugins;
use bevy::log::info;
use bevy::prelude::App;
use bevy_ecs_ldtk_macros::LdtkEntity;
use bevy::prelude::Bundle;
use bevy::prelude::Component;
use bevy_asset::AssetServer;
use bevy_render::prelude::ImagePlugin;
use bevy::prelude::PluginGroup;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use bevy_ecs_ldtk::app::LdtkIntCellAppExt;
use bevy::prelude::IVec2;
use crate::movement::{move_all, move_player_from_input, randomize_movements};

mod wall;
mod constants;
mod player;
mod goat;
mod movement;
mod spawn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_ecs_ldtk::LdtkPlugin)
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoatBundle>("Goat")
        .register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
        .register_ldtk_entity::<EntryPointBundle>("EntryPoint")
        .register_ldtk_int_cell::<WallBundle>(1)
        .init_resource::<LevelWalls>()
        .init_resource::<LevelEntryPoints>()
        .add_systems(
            Update,
            (
                move_player_from_input,
                translate_grid_coords_entities,
                cache_wall_locations,
                cache_entry_point_locations,
                move_all,
                randomize_movements,
                check_goal
            )
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

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/firstmap.ldtk"),
        ..Default::default()
    });
}

fn check_goal(
    level_selection: ResMut<LevelSelection>,
    players: Query<&Transform, (With<Player>, Changed<Transform>)>,
    goals: Query<&GridCoords, With<EntryPoint>>,
    level_entries: Res<LevelEntryPoints>,
) {
    if players
        .iter()
        .zip(goals.iter())
        .any(|(player_grid_coords,_)|level_entries.in_entry_point_with_size(&player_grid_coords.translation.truncate(), 16) )
    {
        info!("Пришли");
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        indices.level += 1;
    }
}