use crate::entities::utils::NextUpdate;
use crate::movement::{Character, MovementSpeed};
use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkSpriteSheetBundle;
use bevy_ecs_ldtk_macros::LdtkEntity;

#[derive(Default, Component)]
pub struct Goat;

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoatBundle {
    goat: Goat,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    // #[grid_coords]
    // grid_coords: GridCoords,
    character: Character,
    movement_speed: MovementSpeed,
    next_speed_update: NextUpdate,
}
