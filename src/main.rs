//! Renders a 2D scene containing a single, moving sprite.

use crate::entities::level_params::LevelCoords;
use crate::entities::level_params::LevelSizes;
use crate::plugins::attack::AttackPlugin;
use crate::plugins::movement::MovementPlugin;
use crate::states::MyAppState;
use crate::systems::caching::level_params::cache_level_params;
use bevy::log::LogPlugin;
use bevy::prelude::GlobalTransform;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::{default, AppExtStates};
use bevy::prelude::{Component, Query, Transform, With, Without};
use bevy::{
    prelude::{App, Camera2dBundle, Commands, PluginGroup, Res, Startup, Update},
    DefaultPlugins,
};
use bevy_asset::AssetServer;
use bevy_ecs_ldtk::LdtkSettings;
use bevy_ecs_ldtk::LevelSpawnBehavior;
use bevy_ecs_ldtk::{
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
    LdtkWorldBundle, LevelSelection,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_render::prelude::ImagePlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
use bincode::{config, Decode, Encode};
use clap::Parser;
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

use entities::goat::GoatBundle;

use crate::entities::fignya::FignyaBundle;
use crate::entities::friendly::Friendly;
use crate::entities::player::Player;
use crate::entities::utils::VisiblyDistance;
use crate::entities::wall::LevelWalls;
use crate::plugins::development::DevelopmentPlugin;
use crate::plugins::game::GamePlugin;
use crate::resources::cursor_position::MyWorldCoords;
use crate::resources::entry_point_destinations::LevelEntryPoints;
use crate::resources::spawn_point::SpawnPointId;
use crate::systems::actions::attack::check_killed;
use crate::systems::actions::attack::check_killed_player;
use crate::systems::actions::attack::move_missiles;
use crate::systems::actions::attack::{attack_player_from_input, randomize_attacks};
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
use crate::systems::health::calculate_health;
use crate::systems::health::spawn_health_bars;
use crate::systems::health::{regen_health, update_health_bars};
use crate::systems::spawn::check_player_on_entry;
use crate::systems::spawn::process_player;
use crate::systems::spawn::MyLevelNeighbors;
use crate::systems::spawn::{cache_neighbor_levels, PlayerSpawnPosition};
use crate::{constants::GRID_SIZE, entities::wall::WallBundle};
mod cli;
mod constants;
mod entities;
mod plugins;
mod resources;
mod states;
mod systems;

/// Этот код нужен будет для сохранения
// #[derive(Encode, Decode, PartialEq, Debug)]
// pub struct GameSave{
//     player_pos: (f32, f32),
// }
//
// impl GameSave {
//     pub fn player_position(&self) -> PlayerSpawnPosition{
//         PlayerSpawnPosition{
//             x: self.player_pos.0,
//             y: self.player_pos.1
//         }
//     }
// }
// let config = config::standard();
// let encoded: Vec<u8> = bincode::encode_to_vec(&game_save, config).unwrap();
// let (decoded, len): (GameSave, usize) = bincode::decode_from_slice(&encoded[..], config).unwrap();

fn main() {
    let args = cli::Args::parse();
    let mut binding = App::new();
    let mut app = binding
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: format!("{},wgpu_core=warn,wgpu_hal=warn,naga=off", args.log_level),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                }),
        )
        .add_plugins(GamePlugin {
            state: MyAppState::InGame,
        })
        .insert_state(MyAppState::InGame);
    if args.dev_tools {
        app = app.add_plugins(DevelopmentPlugin {});
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
    camera.projection.scale = 0.5;
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
