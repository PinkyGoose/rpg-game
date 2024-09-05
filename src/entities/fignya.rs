use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::{EntityInstance, LdtkSpriteSheetBundle};
use bevy_ecs_ldtk_macros::LdtkEntity;

use crate::entities::health::{Health, Regeneration};
use crate::entities::utils::Character;
use crate::entities::utils::MovementSpeed;
use crate::entities::utils::NextUpdate;
use crate::entities::utils::VisiblyDistance;

#[derive(Default, Component)]
pub struct Fignya;

#[derive(Default, Bundle, LdtkEntity)]
pub struct FignyaBundle {
    fignya: Fignya,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    // #[grid_coords]
    // grid_coords: GridCoords,
    health: Health,
    #[from_entity_instance]
    entity_instance: EntityInstance,
    character: Character,
    movement_speed: MovementSpeed,
    next_speed_update: NextUpdate,
    health_regeneration: Regeneration,
    visibly_distance: VisiblyDistance,
}
//
// impl Default for FignyaBundle {
//     fn default() -> Self {
//         let mut rng = rand::thread_rng();
//         let health = rng.gen_range(1..30);
//         let cur_health = rng.gen_range(0..health);
//         info!("у козы {:?}, {:?}", health, cur_health);
//         Self {
//             fignya: Fignya::default(), // Здесь необходимо установить значение по умолчанию для компонента Fignya, если требуется.
//             sprite_bundle: LdtkSpriteSheetBundle::default(), // Устанавливаем значение по умолчанию для sprite_bundle.
//             // health_bar: AllHealthBundle {
//             health: Health { max: health as f32, current: cur_health as f32 },
//             //     health_bar: HealthBar,
//             //     health_bar_background: HealthBarBackground,
//             //
//             character: Character::default(), // Устанавливаем значение по умолчанию для character.
//             movement_speed: MovementSpeed::default(), // Устанавливаем значение по умолчанию для movement_speed.
//             next_speed_update: NextUpdate::default(), // Устанавливаем значение по умолчанию для next_speed_update.
//             health_regeneration: Regeneration(rng.gen()),
//         }
//     }
// }
