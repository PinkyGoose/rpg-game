use crate::constants::MISSILE_SPEED;
use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Added, Commands, DespawnRecursiveExt, Entity, MouseButton, Query, Res, Time, Transform, With, Without};
use bevy::utils::default;
use log::warn;
use rand::Rng;
use crate::entities::friendly::Friendly;

use crate::entities::health::Health;
use crate::entities::missile::{Damage, Missile, MissileBundle};
use crate::entities::player::Player;
use crate::entities::utils::{Character, MovementSpeed, NextAttack, VisiblyDistance};
use crate::entities::wall::LevelWalls;
use crate::resources::cursor_position::MyWorldCoords;

pub fn attack_player_from_input(
    mut commands: Commands,
    players: Query<&mut Transform, With<Player>>,
    mut character: Query<(&Transform, &mut Health), Without<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<MyWorldCoords>,
) {
    let player_translation = match players.get_single() {
        Ok(player) => { player.translation.truncate() }
        Err(e) => {
            warn!("Нет игрока в move_player_from_input {e:?}");
            return;
        }
    };
    let vec_between_cursor = cursor_coords.0 - player_translation;
    if buttons.just_pressed(MouseButton::Left) {

        for (a, mut b) in character.iter_mut() {
            if a.translation.truncate().distance(player_translation) < 50.0 {
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
    if buttons.just_pressed(MouseButton::Right) {
        let missile_bundle = MissileBundle{
            missile: Missile,
            movement_speed: MovementSpeed{
                0: vec_between_cursor.normalize()*MISSILE_SPEED,
            },
            damage: Damage{0:10.},
            transform: Transform{
                translation: player_translation.extend(0.),
                ..default()
            },
        };
        info!("стриляем");
        commands.spawn(missile_bundle);
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

pub fn move_missiles(
    time: Res<Time>,
    mut commands: Commands,
    mut characters: Query<(& Transform, &mut Health), (With<Character>, Without<Player>)>,
    mut missiles: Query<(Entity,&mut Transform, &MovementSpeed, &Damage, &Missile), (With<Missile>, Without<Character>)>,
    level_walls: Res<LevelWalls>,
) {
    for (entity,mut coords,speed, dmg,_) in missiles.iter_mut() {
        // info!("{destination:?}");
        let mut speed = speed.0;
        let destination = coords.translation + speed.extend(0.) * time.delta_seconds();
        if level_walls.in_wall_horizontal_with_size(&destination.truncate(), 0)||level_walls.in_wall_vertical_with_size(&destination.truncate(), 0){
            commands.entity(entity).despawn_recursive()
        }
        for (character_pos, mut health) in characters.iter_mut(){
            let character_pos = character_pos.translation.truncate();
            if destination.truncate().distance(character_pos)<16.{//TODO заменить 16 на что-то поинтереснее. Это 16 - область вокруг существа, что-то вроде хитбокса
                health.current -= dmg.0;
                commands.entity(entity).despawn_recursive();
            }
        }
        coords.translation = destination;
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