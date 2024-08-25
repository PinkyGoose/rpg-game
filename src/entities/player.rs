use crate::{
    movement::{Character, MovementSpeed},
};
use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkSpriteSheetBundle;
use bevy_ecs_ldtk_macros::LdtkEntity;
#[derive(Default, Component)]
pub struct PlayerChild;
#[derive(Default, Component)]
pub struct Player;
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    // #[sprite_sheet_bundle]
    // pub sprite_bundle: LdtkSpriteSheetBundle,
    pub character: Character,
    pub movement_speed: MovementSpeed,
}

