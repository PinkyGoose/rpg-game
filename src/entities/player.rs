use crate::{
    movement::{Character, MovementSpeed},
};
use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkSpriteSheetBundle;
use bevy_ecs_ldtk_macros::LdtkEntity;

#[derive(Default, Component)]
pub struct Player;
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    character: Character,
    movement_speed: MovementSpeed,
}

