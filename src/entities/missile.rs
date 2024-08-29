use bevy::prelude::{Bundle, Component};
use crate::entities::utils::MovementSpeed;

#[derive(Component, Default)]
pub struct Missile;

#[derive(Component, Default)]
pub struct Damage(pub f32);
#[derive(Bundle, Default)]
pub struct MissileBundle{
    missile: Missile,
    movement_speed: MovementSpeed,

}