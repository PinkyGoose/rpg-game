use bevy::prelude::{Added, Commands, Entity, Query, Without};
use bevy_ecs_ldtk::prelude::LdtkFields;
use bevy_ecs_ldtk::EntityInstance;

use crate::entities::friendly::Friendly;
use crate::entities::player::Player;
use crate::entities::utils::Character;

pub fn calculate_friendly(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance), (Added<Character>, Without<Player>)>,
) {
    for (entity, entity_instance) in new_entity_instances.iter() {
        // info!("calculate_friendly {:?}",entity_instance.identifier);
        if let Ok(friendly) = entity_instance.get_enum_field("Friendly") {
            commands.entity(entity).insert(match friendly.as_str() {
                "Friendly" => Friendly::Friend,
                "Enemy" => Friendly::Enemy,
                "Afraid" => Friendly::Afraid,
                _ => Friendly::Neutral,
            });
        }
    }
}
