use bevy::math::Vec2;
use bevy::prelude::Resource;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);