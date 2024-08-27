use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{ Commands, DespawnRecursiveExt, Entity, MouseButton, Query, Res, Transform, With, Without};
use log::warn;

use crate::entities::health::Health;
use crate::entities::player::Player;

pub fn attack_player_from_input(
    players: Query<&mut Transform, With<Player>>,
    mut character: Query<(&mut Transform, &mut Health), Without<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let player_translation = match players.get_single() {
            Ok(player) => { player.translation.truncate() }
            Err(e) => {
                warn!("Нет игрока в move_player_from_input {e:?}");
                return;
            }
        };
        for (a, mut b) in character.iter_mut() {
            if a.translation.truncate().distance(player_translation) < 50.0 {
                info!("attack_translation {player_translation:?}");
                b.current -= 5.0;
                //TODO атаку добавить для других существ
                //TODO кастомизировать урон
            }
        }
        // Left button was pressed
    }
    //TODO анимация атаки
}


// Система для создания полоски здоровья
pub fn check_killed(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform)>,
) {
    for (entity, health, transform) in query.iter() {
        if health.current < 0. {
            //TODO спавним loot в точке transform
            info!("пока что ТИПА спавним лут {transform:?}");
            commands.entity(entity).despawn_recursive();
        }
    }
}