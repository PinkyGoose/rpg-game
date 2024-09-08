//! Renders a 2D scene containing a single, moving sprite.

use bevy::{
    DefaultPlugins,
    prelude::{App, Camera2dBundle, Commands, PluginGroup, Res},
};
use bevy::log::LogPlugin;
use bevy::prelude::{GlobalTransform, OnEnter, OnExit};
use bevy::prelude::{AppExtStates, default};
use bevy::prelude::{Component, Query, Transform, With, Without};
use bevy_asset::AssetServer;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_render::prelude::ImagePlugin;
use clap::Parser;
use iyes_perf_ui::entries::PerfUiBundle;

use crate::constants::GRID_SIZE;
use crate::entities::friendly::Friendly;
use crate::entities::player::Player;
use crate::plugins::attack::AttackPlugin;
use crate::plugins::development::DevelopmentPlugin;
use crate::plugins::game::GamePlugin;
use crate::plugins::movement::MovementPlugin;
use crate::states::MyAppState;
use crate::ui::{activate_main_camera, activate_ui_camera, deactivate_main_camera, deactivate_ui_camera, despawn_ui, setup_ui, toggle_pause_game};
use crate::ui::MyUiPlugin;

mod cli;
mod constants;
mod entities;
mod plugins;
mod resources;
mod states;
mod systems;
pub mod ui;

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
                })
                .disable::<LogPlugin>(),
        )
        .add_plugins(MyUiPlugin {
            state: MyAppState::MainMenu
        })
        .add_plugins(GamePlugin {
            state: MyAppState::InGame,
        })
        .insert_state(MyAppState::MainMenu)
        .add_systems(bevy::app::Update, toggle_pause_game);
    if args.dev_tools {
        app = app.add_plugins(DevelopmentPlugin {});
    }
    app.add_systems(OnEnter(MyAppState::MainMenu), (
        activate_ui_camera,
        setup_ui
    ));
    app.add_systems(OnExit(MyAppState::MainMenu), (
        deactivate_ui_camera,
        despawn_ui
    ));
    app.add_systems(OnEnter(MyAppState::InGame), (
        activate_main_camera
    ));
    app.add_systems(OnExit(MyAppState::InGame), (
        deactivate_main_camera
    ));
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
    camera.camera.is_active = false;
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
