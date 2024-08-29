use std::time::Duration;
use bevy::prelude::{Added, Commands, Entity, Query};
use crate::entities::friendly::Friendly;
use crate::entities::utils::NextAttack;

pub fn insert_enemy_attack_time(
    mut commands: Commands,
    character: Query<(Entity, &Friendly), Added<crate::Friendly>>,
) {
    for (entity, friendly) in character.iter() {
        match friendly {
            Friendly::Enemy => {
                commands.entity(entity).insert(NextAttack { time: Duration::from_secs(0) });
            }
            _ => {}
        }
    }
}