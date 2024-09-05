use crate::constants::MISSILE_SPEED;
use crate::entities::friendly::Friendly;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::{BuildChildren, GlobalTransform};
use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, MouseButton, Query, Res, Time, Transform, With, Without,
};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy_asset::AssetServer;
use log::warn;
use rand::Rng;
use std::time::Duration;

use crate::entities::health::Health;
use crate::entities::missile::{Damage, Missile, MissileBundle};
use crate::entities::player::Player;
use crate::entities::utils::{Character, MovementSpeed, NextAttack, VisiblyDistance};
use crate::entities::wall::LevelWalls;
use crate::resources::cursor_position::MyWorldCoords;

pub fn attack_player_from_input(
    mut commands: Commands,
    players: Query<&mut GlobalTransform, With<Player>>,
    mut character: Query<(&GlobalTransform, &mut Health), Without<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<MyWorldCoords>,
    assets: Res<AssetServer>,
) {
    let (player_translation, z) = match players.get_single() {
        Ok(player) => {
            let z = player.translation().z;
            (player.translation().truncate(), z)
        }
        Err(e) => {
            warn!("Нет игрока в move_player_from_input {e:?}");
            return;
        }
    };
    let vec_between_cursor = cursor_coords.0 - player_translation;
    if buttons.just_pressed(MouseButton::Left) {
        for (a, mut b) in character.iter_mut() {
            if a.translation().truncate().distance(player_translation) < 50.0 {
                let vec_between_creature = a.translation().truncate() - player_translation;
                let angle = vec_between_cursor.angle_between(vec_between_creature);
                let angle = if angle > 0. { angle } else { -angle };
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
        let texture = assets.load("arrow.png");
        // SpriteBundle{
        //     sprite: Default::default(),
        //     transform: Default::default(),
        //     global_transform: Default::default(),
        //     texture: Default::default(),
        //     visibility: Default::default(),
        //     inherited_visibility: Default::default(),
        //     view_visibility: Default::default(),
        // }
        let jija = Transform::from_xyz(player_translation.x, player_translation.y, z)
            .looking_to(Vec3::ZERO, vec_between_cursor.extend(0.));
        let jojo = Transform::from_xyz(player_translation.x, player_translation.y, z);
        info!("jiojioj {:?} {:?}", jija, jojo);
        let missile_bundle = MissileBundle {
            missile: Missile,
            movement_speed: MovementSpeed {
                0: -vec_between_cursor.normalize() * MISSILE_SPEED,
            },
            damage: Damage { 0: 10. },
            // global_transform: GlobalTransform::from(player_translation.extend(2.))
            transform: Transform::from_xyz(player_translation.x, player_translation.y, z)
                .looking_to(Vec3::ZERO, vec_between_cursor.extend(0.)),
            ..default()
        };
        info!("стриляем");
        commands.spawn(missile_bundle).with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture,
                ..default()
            });
        });
    }
    //TODO анимация атаки
}

pub fn randomize_attacks(
    time: Res<Time>,
    mut creatures: Query<
        (&mut NextAttack, &Transform, &VisiblyDistance, &Friendly),
        (With<Character>, Without<Player>),
    >,
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

        if translation.distance(player_translation) < 30.
            && translation.distance(player_translation) < visible.0
            && time_attack.time < time_elapsed
        {
            match friendly {
                Friendly::Enemy => {
                    // let speed_calculated = player_translation - translation;
                    health.current = health.current - 15.; //TODO заменить на урон существа

                    time_attack.time = time_elapsed + Duration::from_secs(rng.gen_range(1..5)); //TODO заменить на время атаки

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
    mut characters: Query<(&GlobalTransform, &mut Health), (With<Character>, Without<Player>)>,
    mut missiles: Query<
        (
            Entity,
            &GlobalTransform,
            &mut Transform,
            &MovementSpeed,
            &Damage,
            &Missile,
        ),
        (With<Missile>, Without<Character>),
    >,
    level_walls: Res<LevelWalls>,
) {
    for (entity, coords_global, mut coords, speed, dmg, _) in missiles.iter_mut() {
        // info!("{destination:?}");
        let speed = speed.0;
        let dest_global = coords_global.translation().truncate() + speed * time.delta_seconds();
        let destination = coords_global.translation().truncate() + speed * time.delta_seconds();
        if level_walls.in_wall_horizontal_with_size(&dest_global, 0)
            || level_walls.in_wall_vertical_with_size(&dest_global, 0)
        {
            commands.entity(entity).despawn_recursive()
        }
        for (character_pos, mut health) in characters.iter_mut() {
            let character_pos = character_pos.translation().truncate();
            if destination.distance(character_pos) < 16. {
                //TODO заменить 16 на что-то поинтереснее. Это 16 - область вокруг существа, что-то вроде хитбокса
                health.current -= dmg.0;
                commands.entity(entity).despawn_recursive();
            }
        }
        //TODO если достаточно далеко, то ремув
        coords.translation += coords_global.translation().with_z(0.) - destination.extend(0.);
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
