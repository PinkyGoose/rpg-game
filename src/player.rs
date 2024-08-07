use bevy::input::ButtonInput;
use bevy::prelude::{Bundle, Component, KeyCode, Query, Res, With};
use bevy_ecs_ldtk::{GridCoords, LdtkSpriteSheetBundle};
use bevy_ecs_ldtk_macros::LdtkEntity;
use crate::movement::{Character, MovementSpeed};
use crate::wall::LevelWalls;


#[derive(Default, Component)]
pub struct Player;
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    character: Character,
    movement_speed: MovementSpeed
}