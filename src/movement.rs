use bevy::input::ButtonInput;
use bevy::prelude::{Component, IVec2, KeyCode, Query, Res, With};
use bevy_ecs_ldtk::GridCoords;
use crate::player::Player;
use crate::wall::LevelWalls;
use rand::Rng;
pub fn move_player_from_input(
    mut players: Query<&mut GridCoords, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    level_walls: Res<LevelWalls>,
) {
    let movement_direction = if input.just_pressed(KeyCode::KeyW) {
        GridCoords::new(0, 1)
    } else if input.just_pressed(KeyCode::KeyA) {
        GridCoords::new(-1, 0)
    } else if input.just_pressed(KeyCode::KeyS) {
        GridCoords::new(0, -1)
    } else if input.just_pressed(KeyCode::KeyD) {
        GridCoords::new(1, 0)
    } else {
        return;
    };

    for mut player_grid_coords in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *player_grid_coords = destination;
        }
    }
}


pub fn move_all(
    mut characters: Query<(&mut GridCoords, &MovementSpeed), With<Character>>,
    level_walls: Res<LevelWalls>,
) {
    let mut rng = rand::thread_rng();


    for (mut coords, mut speed) in characters.iter_mut() {
        let num: u8 = rng.gen_range(0..5);
        let movement_direction=match num {
            0=>GridCoords::new(0, 1),
            1=>GridCoords::new(0, -1),
            2=>GridCoords::new(-1, 0),
            3=>GridCoords::new(1, 0),
            _=> break
        };
        let destination = *coords + movement_direction;
        if !level_walls.in_wall(&destination) {
            *coords = destination;
        }
    }
}

#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Component)]
pub struct MovementSpeed(IVec2);