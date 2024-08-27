use bevy_ecs_ldtk::EntityInstance;
use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::LdtkSpriteSheetBundle;
use bevy_ecs_ldtk_macros::LdtkEntity;

use crate::entities::health::{Health, Regeneration};
use crate::entities::utils::NextUpdate;
use crate::movement::{Character, MovementSpeed};

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
}

// impl Default for GoatBundle {
//     fn default() -> Self {
//         let mut rng = rand::thread_rng();
//         let health = rng.gen_range(1..30);
//         let cur_health = rng.gen_range(0..health);
//         info!("у козы {:?}, {:?}", health, cur_health);
//         Self {
//             goat: Goat::default(), // Здесь необходимо установить значение по умолчанию для компонента Goat, если требуется.
//             sprite_bundle: LdtkSpriteSheetBundle::default(), // Устанавливаем значение по умолчанию для sprite_bundle.
//             // health_bar: AllHealthBundle {
//                 health: Health { max: health as f32, current: cur_health as f32 },
//             //     health_bar: HealthBar,
//             //     health_bar_background: HealthBarBackground,
//             // },
//             character: Character::default(), // Устанавливаем значение по умолчанию для character.
//             movement_speed: MovementSpeed::default(), // Устанавливаем значение по умолчанию для movement_speed.
//             next_speed_update: NextUpdate::default(), // Устанавливаем значение по умолчанию для next_speed_update.
//             health_regeneration: Regeneration(rng.gen()),
//         }
//     }
// }