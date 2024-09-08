use std::time::Duration;

use bevy::log::warn;
use bevy::prelude::{Changed, GlobalTransform};
use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Query, Res, Time, Transform, Vec2, With, Without},
};
use bevy_spritesheet_animation::component::SpritesheetAnimation;
use bevy_spritesheet_animation::library::SpritesheetLibrary;
use rand::Rng;

use crate::entities::friendly::Friendly;
use crate::entities::utils::NextUpdate;
use crate::entities::utils::VisiblyDistance;
use crate::entities::utils::{Character, MovementSpeed};
use crate::entities::wall::{DirectionX, DirectionY};
use crate::{
    constants::{GOAT_SPEED, PLAYER_SPEED},
    entities::player::Player,
    entities::wall::LevelWalls,
};

pub fn move_player_from_input(
    mut players: Query<(&mut MovementSpeed, &mut SpritesheetAnimation), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    library: Res<SpritesheetLibrary>,
) {
    let mut movement_direction = Vec2::new(-0., 0.);
    if input.pressed(KeyCode::KeyW) {
        movement_direction += Vec2::new(0., 1.);
    }
    if input.pressed(KeyCode::KeyA) {
        movement_direction += Vec2::new(-1., 0.);
    }
    if input.pressed(KeyCode::KeyS) {
        movement_direction += Vec2::new(0., -1.);
    }
    if input.pressed(KeyCode::KeyD) {
        movement_direction += Vec2::new(1., 0.);
    }

    for (mut speed, mut animation) in players.iter_mut() {
        if movement_direction.x < 0. {
            if let Some(id) = library.animation_with_name("run_left") {
                animation.animation_id = id;
            }
        } else if movement_direction.x > 0. {
            if let Some(id) = library.animation_with_name("run_right") {
                animation.animation_id = id;
            }
        } else if let Some(id) = library.animation_with_name("archer_idle") {
            animation.animation_id = id;
        }

        speed.0 = if movement_direction == Vec2::new(-0., 0.) {
            movement_direction
        } else {
            movement_direction.normalize() * PLAYER_SPEED
        };
    }
}

pub fn randomize_movements(
    time: Res<Time>,
    mut creatures: Query<
        (
            &mut MovementSpeed,
            &mut NextUpdate,
            &Transform,
            &VisiblyDistance,
            &Friendly,
        ),
        (With<Character>, Without<Player>),
    >,
    player: Query<&Transform, With<Player>>,
) {
    let time_elapsed = time.elapsed();
    let mut rng = rand::thread_rng();
    let player_translation = match player.get_single() {
        Ok(p) => p.translation.truncate(),
        Err(e) => {
            warn!("Кажись тут нет персонажа, но вот подробнее {e:?}");
            return;
        }
    };
    for (mut speed, mut time_update, transform, visible, friendly) in creatures.iter_mut() {
        let translation = transform.translation.truncate();

        if visible.0 > player_translation.distance(translation) {
            match friendly {
                Friendly::Enemy => {
                    let speed_calculated = player_translation - translation;
                    if speed_calculated.length() > 16. {
                        //TODO тут будет дальность атаки
                        // info!("Должен подходить {speed_calculated:?}");
                        speed.0 = speed_calculated.normalize() * GOAT_SPEED; //TODO заменить на Movement_Speed
                    } else {
                        speed.0 = speed_calculated.normalize() * 0.; //TODO заменить на Movement_Speed
                    }
                    time_update.time = time_elapsed + Duration::from_secs(rng.gen_range(1..5));
                    continue;
                }

                Friendly::Afraid => {
                    let speed_calculated = translation - player_translation;
                    // info!("Должен убегать {speed_calculated:?}");
                    speed.0 = speed_calculated.normalize() * GOAT_SPEED; //TODO заменить на Movement_Speed
                    time_update.time = time_elapsed + Duration::from_secs(rng.gen_range(1..5));
                    continue;
                }
                Friendly::Friend => {}
                Friendly::Neutral => {}
            }
        }
        if time_update.time < time_elapsed {
            let speed_calculated = Vec2::new(rng.gen::<f32>(), rng.gen::<f32>())
                - Vec2::new(rng.gen::<f32>(), rng.gen::<f32>());
            speed.0 = speed_calculated.normalize() * GOAT_SPEED; //TODO заменить на Movement_Speed
            time_update.time = time_elapsed + Duration::from_secs(rng.gen_range(1..5));
        }
    }
}

pub fn move_all(
    time: Res<Time>,
    mut characters: Query<
        (&mut Transform, &GlobalTransform, &MovementSpeed),
        (With<Character>, Changed<GlobalTransform>),
    >,
    level_walls: Res<LevelWalls>,
) {
    for (mut coords, coords_global, speed) in characters.iter_mut() {
        let speed = speed.0;
        let dest_global = coords_global.translation().truncate() + speed * time.delta_seconds();
        let direction_x = if speed.x == 0. {
            DirectionX::None
        } else if speed.x < 0. {
            DirectionX::Left
        } else {
            DirectionX::Right
        };
        let direction_y = if speed.y == 0. {
            DirectionY::None
        } else if speed.y < 0. {
            DirectionY::Down
        } else {
            DirectionY::Up
        };
        let (direction_x, direction_y) = level_walls.get_access_to_go(
            &coords_global.translation().truncate(),
            &dest_global,
            direction_x,
            direction_y,
        );
        let speed = Vec2::new(
            match direction_x {
                DirectionX::None => 0.,
                _ => speed.x,
            },
            match direction_y {
                DirectionY::None => 0.,
                _ => speed.y,
            },
        );
        let destination = coords.translation + speed.extend(0.) * time.delta_seconds();
        coords.translation = destination;
    }
}
