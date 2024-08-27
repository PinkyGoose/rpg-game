use bevy::prelude::{Added, Commands, Entity, Query, Without};
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::LdtkFields;

use crate::entities::friendly::{Afraid, Enemy, Friendly, Neytral};
use crate::entities::player::Player;
use crate::movement::Character;

pub fn calculate_friendly(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance), (Added<Character>, Without<Player>)>,
)
{
    for (entity, entity_instance) in new_entity_instances.iter() {

        // info!("calculate_friendly {:?}",entity_instance.identifier);
        if let Ok(friendly) = entity_instance.get_enum_field("Friendly") {
            match friendly.as_str() {
                "Friendly" => {
                    commands
                        .entity(entity).insert(
                        Friendly
                    );
                }
                "Enemy" => {
                    commands
                        .entity(entity).insert(
                        Enemy
                    );
                }
                "Afraid" => {
                    commands
                        .entity(entity).insert(
                        Afraid
                    );
                }
                _ => {
                    commands
                        .entity(entity).insert(
                        Neytral
                    );
                }
            }
        }
    }
}