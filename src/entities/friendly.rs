use bevy::prelude::ReflectComponent;
use bevy::prelude::{Component, Reflect};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

#[derive(Reflect, Default, Component, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub enum Friendly {
    Enemy,

    Friend,
    #[default]
    Neutral,
    Afraid,
}
