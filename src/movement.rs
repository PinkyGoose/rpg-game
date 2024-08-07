use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Component, IVec2, KeyCode, Query, Res, Time, Transform, With};
use bevy_ecs_ldtk::GridCoords;
use glam::IVec3;
use crate::player::Player;
use crate::wall::LevelWalls;
use bevy::prelude::Vec3;
use rand::Rng;
use crate::constants::GRID_SIZE;

pub fn move_player_from_input(
    mut players: Query<&mut MovementSpeed, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let mut movement_direction = Vec3::new(-0., 0., 0.);
    if input.pressed(KeyCode::KeyW) {
        movement_direction+=Vec3::new(0.,50.,0.);
    }
    if input.pressed(KeyCode::KeyA) {
        movement_direction+=Vec3::new(-50.,0.,0.);
    }
        if input.pressed(KeyCode::KeyS) {
            movement_direction+=Vec3::new(0.,-50.,0.);
    }
        if input.pressed(KeyCode::KeyD) {
            movement_direction+=Vec3::new(50.,0.,0.);
    }

    for mut speed in players.iter_mut() {
            speed.0 = movement_direction;
    }
}


pub fn move_all(
    time: Res<Time>,
    mut characters: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    level_walls: Res<LevelWalls>,
) {
    let mut rng = rand::thread_rng();


    for (mut coords, speed) in characters.iter_mut() {
        // let num: u8 = rng.gen_range(0..5);
        // let movement_direction=match num {
        //     0=>GridCoords::new(0, 1),
        //     1=>GridCoords::new(0, -1),
        //     2=>GridCoords::new(-1, 0),
        //     3=>GridCoords::new(1, 0),
        //     _=> break
        // };
        let destination = coords.translation + speed.0* time.delta_seconds();

        if !level_walls.in_wall_with_size(&destination.truncate(), 32) {

            coords.translation = destination;
        }
    }
}

#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Component, Debug)]
pub struct MovementSpeed(Vec3);