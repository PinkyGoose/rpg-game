use std::time::Duration;
use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Default, Component, Debug)]
pub struct NextUpdate {
    pub time: Duration,
}

#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Component, Debug)]
pub struct MovementSpeed(pub Vec2);

#[derive(Default, Component, Debug)]
pub struct VisiblyDistance(pub f32);