use crate::entities::friendly::Friendly;
use crate::entities::utils::NextAttack;
use bevy::prelude::{Added, Commands, Entity, Query};
use std::time::Duration;

pub fn insert_enemy_attack_time(
    mut commands: Commands,
    character: Query<(Entity, &Friendly), Added<crate::Friendly>>,
) {
    for (entity, friendly) in character.iter() {
        match friendly {
            Friendly::Enemy => {
                commands.entity(entity).insert(NextAttack {
                    time: Duration::from_secs(0),
                });
            }
            _ => {}
        }
    }
}
