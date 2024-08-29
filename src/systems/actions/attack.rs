use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Added, Commands, DespawnRecursiveExt, Entity, MouseButton, Query, Res, Time, Transform, With, Without};
use log::warn;
use rand::Rng;
use crate::entities::friendly::Friendly;

use crate::entities::health::Health;
use crate::entities::player::Player;
use crate::entities::utils::{Character, NextAttack, VisiblyDistance};
use crate::resources::cursor_position::MyWorldCoords;

pub fn attack_player_from_input(
    players: Query<&mut Transform, With<Player>>,
    mut character: Query<(&Transform, &mut Health), Without<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<MyWorldCoords>,
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
                let vec_between_cursor = cursor_coords.0 - player_translation;
                let vec_between_creature = a.translation.truncate() - player_translation;
                let angle = vec_between_cursor.angle_between(vec_between_creature);
                let angle = if angle > 0. {
                    angle
                } else {
                    -angle
                };
                if angle * 57.3 < 15. {
                    b.current -= 5.0;
                }

                // }
                //TODO атаку добавить для других существ
                //TODO кастомизировать урон
                //TODO добавить разное поведение при разных типах атак и при разном оружии
            }
        }
        // Left button was pressed
    }
    //TODO анимация атаки
}

pub fn randomize_attacks(
    time: Res<Time>,
    mut creatures: Query<(&mut NextAttack, &Transform, &VisiblyDistance, &Friendly), (With<Character>, Without<Player>)>,
    mut player: Query<(&Transform, &mut Health), With<Player>>,
) {
    let time_elapsed = time.elapsed();
    let mut rng = rand::thread_rng();
    let (player_translation, mut health) = match player.get_single_mut() {
        Ok((p, health)) => (p.translation.truncate(), health),
        Err(e) => {
            bevy::log::warn!("Кажись тут нет персонажа, но вот подробнее {e:?}");
            return;
        }
    };
    for (mut time_attack, transform, visible, friendly) in creatures.iter_mut() {
        let translation = transform.translation.truncate();

        if translation.distance(player_translation) < 30.&&translation.distance(player_translation) < visible.0 &&time_attack.time<time_elapsed {
            match friendly {
                Friendly::Enemy => {

                    // let speed_calculated = player_translation - translation;
                    health.current = health.current-15.;//TODO заменить на урон существа

                    time_attack.time = time_elapsed + Duration::from_secs(rng.gen_range(1..5));//TODO заменить на время атаки

                    continue;
                }
                _ => {}
            }
        }
    }
}



// Система для создания полоски здоровья
pub fn check_killed(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform), Without<Player>>,
) {
    for (entity, health, transform) in query.iter() {
        if health.current < 0. {
            //TODO спавним loot в точке transform
            info!("пока что ТИПА спавним лут {transform:?}");
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Система для создания полоски здоровья
pub fn check_killed_player(
    mut commands: Commands,
    mut query: Query<(Entity, &Health, &Transform), With<Player>>,
) {
    if let Ok((entity, health, transform)) = query.get_single_mut() {
        if health.current < 0. {
            //TODO реализовать смерть игрока
            info!("Тут должно быть что-то связанное со смертью игрока {transform:?}");
            commands.entity(entity).despawn_recursive();
        }
    }
}