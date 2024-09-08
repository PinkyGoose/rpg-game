use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{default, IntoSystemConfigs, States};
use bevy_ecs_ldtk::{LdtkSettings, LevelSelection, LevelSpawnBehavior};
use bevy_ecs_ldtk::app::{LdtkEntityAppExt, LdtkIntCellAppExt};
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;

use crate::plugins::cache::CachePlugin;
use crate::plugins::visual::VisualPlugin;
use crate::systems::animation::spawn_animations;
use crate::systems::spawn::{MyLevelNeighbors, PlayerSpawnPosition, process_player};
use crate::MovementPlugin;
use crate::{setup, AttackPlugin};
use crate::entities::fignya::FignyaBundle;
use crate::entities::friendly::Friendly;
use crate::entities::goat::GoatBundle;
use crate::entities::level_params::{LevelCoords, LevelSizes};
use crate::entities::utils::VisiblyDistance;
use crate::entities::wall::{LevelWalls, WallBundle};
use crate::resources::cursor_position::MyWorldCoords;
use crate::resources::entry_point_destinations::LevelEntryPoints;
use crate::resources::spawn_point::SpawnPointId;

pub struct GamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttackPlugin {
            state: self.state.clone(),
        })
        .add_plugins(MovementPlugin {
            state: self.state.clone(),
        })
        .add_plugins(CachePlugin {
            state: self.state.clone(),
        })
        .add_plugins(VisualPlugin {
            state: self.state.clone(),
        }).add_plugins(bevy_ecs_ldtk::LdtkPlugin)
            .add_plugins(SpritesheetAnimationPlugin)
            .insert_resource(LevelSelection::iid("bbd618c0-4ce0-11ef-9196-9768dcadd1bb"))
            .insert_resource(MyWorldCoords::default())
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .register_ldtk_entity::<GoatBundle>("Goat")
            .register_ldtk_entity::<FignyaBundle>("Fignya")
            .register_ldtk_int_cell::<WallBundle>(1)
            .init_resource::<LevelWalls>()
            .register_type::<SpawnPointId>()
            .register_type::<VisiblyDistance>()
            .register_type::<Friendly>()
            .init_resource::<LevelEntryPoints>()
            .insert_resource(SpawnPointId(None))
            .insert_resource(LevelSizes::default())
            .insert_resource(LevelCoords::default())
            .insert_resource(MyLevelNeighbors::default())
            .insert_resource(PlayerSpawnPosition { x: 100.0, y: -100. })
        .add_systems(
            Startup,
            (
                setup,
                spawn_animations,
                process_player.after(spawn_animations),
            ),
        );
    }
}
