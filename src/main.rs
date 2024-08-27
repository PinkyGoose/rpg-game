//! Renders a 2D scene containing a single, moving sprite.

use crate::entities::friendly::Friendly;
use crate::systems::caching::visible_distanse::calculate_visible;
use crate::systems::health::calculate_health;
use crate::systems::caching::friendly::calculate_friendly;
use crate::entities::fignya::FignyaBundle;
use bevy::{
    DefaultPlugins,
    prelude::{
        App, Camera2dBundle, Commands, PluginGroup, Res, Startup
        , Update,
    },
};
use bevy::prelude::IntoSystemConfigs;
use bevy_asset::AssetServer;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt}
    , LdtkWorldBundle, LevelSelection,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_render::prelude::ImagePlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
use clap::Parser;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

use entities::goat::GoatBundle;

use crate::{
    constants::GRID_SIZE
    ,
    entities::spawn::{
        EntryPointBundle, LevelEntryPoints,
        SpawnPointId, UnresolvedIdRef,
    },
    entities::wall::{LevelWalls, WallBundle},
    movement::{move_all, move_player_from_input, randomize_movements},
};
use crate::entities::spawn::spawn_player;
use crate::entities::spawn::SpawnPointBundle;
use crate::entities::utils::VisiblyDistance;
use crate::systems::animation::spawn_animations;
use crate::systems::caching::coords::translate_grid_coords_entities;
use crate::systems::caching::entry::cache_entry_point_locations;
use crate::systems::caching::wall::cache_wall_locations;
use crate::systems::health::{regen_health, update_health_bars};
use crate::systems::health::spawn_health_bars;
use crate::systems::spawn::check_player_on_entry;
use crate::systems::spawn::process_player;

mod constants;
mod movement;
mod entities;
mod systems;
mod cli;


fn main() {
    let args = cli::Args::parse();
    let mut binding = App::new();
    let mut app = binding
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy_ecs_ldtk::LdtkPlugin)
        .add_plugins(SpritesheetAnimationPlugin)
        .insert_resource(LevelSelection::iid("bbd618c0-4ce0-11ef-9196-9768dcadd1bb"))
        // .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoatBundle>("Goat")
        .register_ldtk_entity::<FignyaBundle>("Fignya")
        .register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
        .register_ldtk_entity::<EntryPointBundle>("EntryPoint")
        .register_ldtk_int_cell::<WallBundle>(1)
        .init_resource::<LevelWalls>()
        .register_type::<SpawnPointId>()
        .register_type::<UnresolvedIdRef>()
        .register_type::<VisiblyDistance>()
        .register_type::<Friendly>()
        .register_type::<SpawnPointId>()
        .init_resource::<LevelEntryPoints>()
        // .init_resource::<VisiblyDistance>()
        // .init_resource::<Friendly>()
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
                update_health_bars,
                spawn_health_bars,
                regen_health,
                calculate_friendly,
                calculate_health,
                calculate_visible

            ),
        )
        .add_systems(Startup,
                     (
                         spawn_animations, setup
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


