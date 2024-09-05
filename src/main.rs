//! Renders a 2D scene containing a single, moving sprite.

use crate::systems::caching::level_params::cache_level_params;
use crate::entities::level_params::LevelCoords;
use crate::entities::level_params::LevelSizes;
use bevy::{
    DefaultPlugins,
    prelude::{
        App, Camera2dBundle, Commands, PluginGroup, Res, Startup
        , Update,
    },
};
use bevy::prelude::{Component, Query, Transform, With, Without};
use bevy::prelude::default;
use bevy::prelude::GlobalTransform;
use bevy::prelude::IntoSystemConfigs;
use bevy_asset::AssetServer;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt}
    , LdtkWorldBundle, LevelSelection,
};
use bevy_ecs_ldtk::LdtkSettings;
use bevy_ecs_ldtk::LevelSpawnBehavior;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_render::prelude::ImagePlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
use clap::Parser;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

use entities::goat::GoatBundle;

use crate::{
    constants::GRID_SIZE,
    entities::wall::WallBundle,
};
use crate::entities::fignya::FignyaBundle;
use crate::entities::friendly::Friendly;
use crate::entities::player::Player;
use crate::entities::utils::VisiblyDistance;
use crate::entities::wall::LevelWalls;
use crate::resources::cursor_position::MyWorldCoords;
use crate::resources::entry_point_destinations::LevelEntryPoints;
use crate::resources::spawn_point::SpawnPointId;
use crate::systems::actions::attack::{attack_player_from_input, randomize_attacks};
use crate::systems::actions::attack::check_killed;
use crate::systems::actions::attack::check_killed_player;
use crate::systems::actions::attack::move_missiles;
use crate::systems::animation::spawn_animations;
use crate::systems::caching::attack::insert_enemy_attack_time;
use crate::systems::caching::coords::translate_grid_coords_entities;
use crate::systems::caching::cursor::my_cursor_system;
use crate::systems::caching::friendly::calculate_friendly;
use crate::systems::caching::movement::move_all;
use crate::systems::caching::movement::move_player_from_input;
use crate::systems::caching::movement::randomize_movements;
use crate::systems::caching::visible_distanse::calculate_visible;
use crate::systems::caching::wall::cache_wall_locations;
use crate::systems::health::{regen_health, update_health_bars};
use crate::systems::health::calculate_health;
use crate::systems::health::spawn_health_bars;
use crate::systems::spawn::{cache_neighbor_levels, PlayerSpawnPosition};
use crate::systems::spawn::check_player_on_entry;
use crate::systems::spawn::MyLevelNeighbors;
use crate::systems::spawn::process_player;

mod constants;
mod entities;
mod systems;
mod cli;
mod resources;


fn main() {
    let args = cli::Args::parse();
    let mut binding = App::new();
    let mut app = binding
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_ecs_ldtk::LdtkPlugin)
        .add_plugins(SpritesheetAnimationPlugin)
        .insert_resource(LevelSelection::iid("bbd618c0-4ce0-11ef-9196-9768dcadd1bb"))
        .insert_resource(MyWorldCoords::default())
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true
            },
            ..default()
        })
        // .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoatBundle>("Goat")
        .register_ldtk_entity::<FignyaBundle>("Fignya")
        // .register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
        // .register_ldtk_entity::<EntryPointBundle>("EntryPoint")
        .register_ldtk_int_cell::<WallBundle>(1)
        // .register_ldtk_int_cell_for_layer::<WallBundle>(1)
        // .register_ldtk_int_cell::<WallBundle>(1)
        // .register_ldtk_int_cell::<WallBundle>(1)
        // .register_default_ldtk_int_cell()
        .init_resource::<LevelWalls>()
        .register_type::<SpawnPointId>()
        // .register_type::<UnresolvedIdRef>()
        .register_type::<VisiblyDistance>()
        .register_type::<Friendly>()
        .init_resource::<LevelEntryPoints>()
        // .init_resource::<VisiblyDistance>()
        // .init_resource::<Friendly>()
        .insert_resource(SpawnPointId(None))
        .insert_resource(LevelSizes::default())
        .insert_resource(LevelCoords::default())
        .insert_resource(MyLevelNeighbors::default())
        .insert_resource(PlayerSpawnPosition { x: 100.0, y: 0. })
        .add_systems(
            Update,
            (
                move_player_from_input,
                translate_grid_coords_entities,
                cache_wall_locations.after(cache_neighbor_levels),
                cache_level_params,
                move_all,
                randomize_movements,
                check_player_on_entry,
                // spawn_player.after(process_player),
                update_health_bars,
                spawn_health_bars,
                regen_health,
                calculate_friendly,
                calculate_health,
                calculate_visible,
                attack_player_from_input,
                check_killed,
                my_cursor_system,
                randomize_attacks,
                insert_enemy_attack_time,
            ),
        )
        .add_systems(
            Update,
            (
                check_killed_player,
                move_missiles,
                show_character,
                cache_neighbor_levels.after(cache_level_params)
            ),
        )
        .add_systems(Startup,
                     (
                         spawn_animations, setup,
                     process_player.after(spawn_animations),
                     ),
        );
    if args.dev_tools {
        app = app.add_systems(Startup, dev_plug)
            .add_plugins(PerfUiPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    }
    app.run();
}

pub fn dev_plug(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
}


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.3;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn((camera, MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("map/firstmap.ldtk"),
        ..Default::default()
    });
}

fn show_character(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    player: Query<&GlobalTransform, (With<Player>, Without<MainCamera>)>,
) {
    if let Ok(mut camera) = camera.get_single_mut() {
        if let Ok(player) = player.get_single() {

            camera.translation = player.translation();
        }
    }
}
