use bevy::prelude::{ Component};

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarBackground;
#[derive(Component)]
pub struct Regeneration(pub(crate) f32);
