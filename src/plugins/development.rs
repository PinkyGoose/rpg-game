use crate::dev_plug;
use bevy::app::{App, Plugin, Startup};
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
