use bevy::log::warn;
use bevy::prelude::{Added, Query, Without};
use bevy_ecs_ldtk::EntityInstance;
use crate::entities::player::Player;
use crate::entities::utils::{Character, VisiblyDistance};
use bevy_ecs_ldtk::prelude::LdtkFields;
pub fn calculate_visible(
    mut new_entity_instances: Query<(&EntityInstance, &mut VisiblyDistance), (Added<Character>, Without<Player>)>,
)
{
    for (entity_instance, mut visible) in new_entity_instances.iter_mut(){
        if let Ok(dist) = entity_instance.get_float_field("visible_distance"){
            visible.0 = *dist
        }
        else{
            warn!("не прописана дистанция видимости  {:?}", entity_instance.identifier);
        }

    }
}