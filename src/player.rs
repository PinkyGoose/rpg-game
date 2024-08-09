use bevy::input::ButtonInput;
use bevy::prelude::{Added, Bundle, Commands, Component, Entity, EventReader, KeyCode, Query, Res, ResMut, Transform, With, Without};
use bevy_asset::AssetServer;
use bevy_ecs_ldtk::{EntityIid, EntityInstance, GridCoords, LdtkSpriteSheetBundle, LevelEvent};
use bevy_ecs_ldtk_macros::LdtkEntity;
use crate::movement::{Character, MovementSpeed};
use crate::spawn::{EntryPoint, SpawnPoint, SpawnPointId, UnresolvedIdRef};
use crate::wall::LevelWalls;


#[derive(Default, Component)]
pub struct Player;
#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: LdtkSpriteSheetBundle,
    // #[grid_coords]
    // grid_coords: GridCoords,
    character: Character,
    movement_speed: MovementSpeed,

    // transform: Transform,
}


pub fn spawn_player(
    mut level_events: EventReader<LevelEvent>,
    mut player: Query<&mut Transform, With<Player>>,
    spawn_points: Query<(&EntityIid, &Transform), (With<SpawnPoint>, Without<Player>)>,
    mut spawn_point: ResMut<SpawnPointId>
)
{
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(_) = level_event {
                if let Some(point) = spawn_point.0.clone(){
                    for (spawn, transform) in spawn_points.iter(){
                        if *spawn==point{

                            if let Ok(mut player_coords) = player.get_single_mut(){
                                player_coords.translation = transform.translation;
                            }
                        }
                    }
                    spawn_point.0 = None;
                    return;
                }
        }}

}