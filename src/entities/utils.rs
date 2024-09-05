use bevy::math::Vec2;
use bevy::prelude::ReflectComponent;
use bevy::prelude::{Component, Reflect};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use std::time::Duration;
#[derive(Default, Component, Debug)]
pub struct NextUpdate {
    pub time: Duration,
}
#[derive(Default, Component, Debug)]
pub struct NextAttack {
    pub time: Duration,
}
#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Component, Debug)]
pub struct MovementSpeed(pub Vec2);

#[derive(Reflect, Default, Component, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct VisiblyDistance(pub f32);
