use crate::systems::actions::attack::{
    attack_player_from_input, check_killed, check_killed_player, move_missiles, randomize_attacks,
};
use crate::MovementPlugin;
use crate::{dev_plug, AttackPlugin};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, States};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::PerfUiPlugin;

pub struct DevelopmentPlugin {}

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dev_plug)
            .add_plugins(PerfUiPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);
    }
}
