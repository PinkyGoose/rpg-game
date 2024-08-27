
use crate::entities::utils::MovementSpeed;
use crate::entities::utils::Character;
use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk_macros::LdtkEntity;
#[derive(Default, Component)]
pub struct PlayerChild;
#[derive(Default, Component)]
pub struct Player;
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub character: Character,
    pub movement_speed: MovementSpeed,
}

