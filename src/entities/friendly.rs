use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component)]
pub struct Friendly;
#[derive(Component)]
pub struct Neytral;
#[derive(Component)]
pub struct Afraid;