use bevy::prelude::Component;


#[derive(Component, Default)]
pub enum Friendly{
    Enemy,

Friend,
#[default]
Neutral,
Afraid
}