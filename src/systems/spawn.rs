use crate::resources::entry_point_destinations::LevelEntryPoints;
use crate::resources::spawn_point::SpawnPointId;
use bevy::hierarchy::BuildChildren;
use bevy::math::UVec2;
use bevy::prelude::{Added, Changed, Commands, default, Entity, Query, Res, ResMut, SpriteBundle, TextureAtlas, TextureAtlasLayout, Transform, With};
use bevy_asset::{Assets, AssetServer};
use bevy_ecs_ldtk::{EntityInstance, LevelSelection};
use bevy_spritesheet_animation::component::SpritesheetAnimation;
use bevy_spritesheet_animation::library::SpritesheetLibrary;

use crate::entities::health::{Health, Regeneration};
use crate::entities::player::{Player, PlayerBundle, PlayerChild};

pub fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance, &Transform), Added<EntityInstance>>,
    library: ResMut<SpritesheetLibrary>,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
)
{
    if let Some((entity, _, transform)) = new_entity_instances.iter().find(|(_,s2,_)|{
        if s2.identifier == "Player".to_string(){
            return true
        }
        false
    }) {
        // if entity_instance.identifier == "Player".to_string() {
        //     info!("переписываем игрока");

            let texture = assets.load("archer.png");

            let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(32, 32),
                3,
                3,
                None,
                None,
            ));
            commands
                .entity(entity)
                // .insert(Player)
                .insert((SpriteBundle {
                    texture,
                    transform: *transform,
                    ..default()
                }, TextureAtlas {
                    layout,
                    ..default()
                }, SpritesheetAnimation::from_id(library.animation_with_name("archer_idle").unwrap()),
                         PlayerBundle {
                             ..default()
                         },
                         Health {
                             current: 34.0, // Изначальное значение здоровья
                             max: 100.0,
                         },
                         Regeneration(8.),
                ))
                .with_children(|commands| {
                    commands.spawn(PlayerChild);
                });
        }
    // }
}
pub fn check_player_on_entry(
    mut spawn_point_id: ResMut<SpawnPointId>,
    mut level_selection: ResMut<LevelSelection>,
    players: Query<&Transform, (With<Player>, Changed<Transform>)>,
    level_entries: Res<LevelEntryPoints>,
) {
    let mut dest = None;
    for player_grid_coords in players.iter() {
        if let Some(level) =
            level_entries.in_entry_point_with_size(&player_grid_coords.translation.truncate(), 16)
        {
            dest = Some(level);
            spawn_point_id.0 = Some(level.spawn_point.clone());
        }
    }
    if let Some(dest) = dest {
        *level_selection = LevelSelection::iid(dest.level.clone())
    }
}