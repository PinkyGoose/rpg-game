
use crate::goat::Goat;
use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Component, IVec2, KeyCode, Query, Res, Time, Transform, With, Without};
use crate::player::Player;
use crate::wall::LevelWalls;
use bevy::prelude::Vec3;
use rand::Rng;
use crate::constants::{GOAT_SPEED, PLAYER_SPEED};
use crate::goat::NextUpdate;
use bevy::prelude::Vec2;


pub fn move_player_from_input(
    mut players: Query<&mut MovementSpeed, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {

    let mut movement_direction = Vec2::new(-0., 0.);
    if input.pressed(KeyCode::KeyW) {
        movement_direction+=Vec2::new(0.,1.);
    }
    if input.pressed(KeyCode::KeyA) {
        movement_direction+=Vec2::new(-1.,0.);
    }
    if input.pressed(KeyCode::KeyS) {
            movement_direction+=Vec2::new(0.,-1.);
    }
    if input.pressed(KeyCode::KeyD) {
            movement_direction+=Vec2::new(1.,0.);
    }

    for mut speed in players.iter_mut() {
            speed.0 = if movement_direction==Vec2::new(-0., 0.){
                movement_direction
            } else {movement_direction.normalize()*PLAYER_SPEED};
    }
}

pub fn randomize_movements(time: Res<Time>,
    mut players: Query<(&mut MovementSpeed, &mut NextUpdate), (With<Character>, Without<Player>)>,
) {

    let time_elapsed =  time.elapsed();
    let mut rng = rand::thread_rng();

    for (mut speed, mut time_update) in players.iter_mut() {
        if time_update.time < time_elapsed{
            let speed_calculated = Vec2::new(rng.gen::<f32>(),rng.gen::<f32>())-Vec2::new(rng.gen::<f32>(),rng.gen::<f32>());
            speed.0 =speed_calculated.normalize()*GOAT_SPEED;
            time_update.time = time_elapsed+Duration::from_secs(rng.gen_range(1..5));
        }
    }
}

pub fn move_all(
    time: Res<Time>,
    mut characters: Query<(&mut Transform, &MovementSpeed), With<Character>>,
    level_walls: Res<LevelWalls>,
) {


    for (mut coords, speed) in characters.iter_mut() {

        let destination = coords.translation + speed.0.extend(0.)* time.delta_seconds();
        // info!("{destination:?}");
        if !level_walls.in_wall_with_size(&destination.truncate(), 16) {
            // info!("{destination:?}\t{speed:?}");

            coords.translation = destination;
        }
    }
}

#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Component, Debug)]
pub struct MovementSpeed(Vec2);