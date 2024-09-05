use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::LdtkSpriteSheetBundle;
use bevy_ecs_ldtk_macros::LdtkEntity;

use crate::entities::health::{Health, Regeneration};
use crate::entities::utils::Character;
use crate::entities::utils::MovementSpeed;
use crate::entities::utils::NextUpdate;
use crate::entities::utils::VisiblyDistance;

#[derive(Default, Component)]
pub struct Goat;

#[derive(Bundle, LdtkEntity, Default)]
pub struct GoatBundle {
    goat: Goat,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    // #[grid_coords]
    // grid_coords: GridCoords,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    health: Health,
    character: Character,
    movement_speed: MovementSpeed,
    next_speed_update: NextUpdate,
    health_regeneration: Regeneration,
    visibly_distance: VisiblyDistance,
}
