use bevy::prelude::Component;

#[derive(Default, Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthBarBackground;
#[derive(Default, Component)]
pub struct Regeneration(pub f32);
