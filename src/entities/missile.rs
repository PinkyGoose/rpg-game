use bevy::prelude::{Bundle, Component, Transform};
use crate::entities::utils::MovementSpeed;

#[derive(Component, Default)]
pub struct Missile;

#[derive(Component, Default)]
pub struct Damage(pub f32);
#[derive(Bundle, Default)]
pub struct MissileBundle{
    pub missile: Missile,
    pub movement_speed: MovementSpeed,
    pub damage: Damage,
    pub transform: Transform
}