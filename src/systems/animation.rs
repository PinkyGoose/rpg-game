use bevy::log::info;
use bevy::prelude::{default, SpriteBundle,  TextureAtlas, Transform};
use bevy::prelude::{Added, BuildChildren, Commands, Entity, Query, Res, ResMut, TextureAtlasLayout, UVec2};
use bevy_asset::{Assets, AssetServer};
use bevy_ecs_ldtk::EntityInstance;
use bevy_spritesheet_animation::library::SpritesheetLibrary;
use bevy_spritesheet_animation::prelude::{AnimationRepeat, Spritesheet, SpritesheetAnimation};
use crate::entities::player::{PlayerBundle, PlayerChild};
pub fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance, &Transform), Added<EntityInstance>>,
    library: ResMut<SpritesheetLibrary>,
    assets: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
)
{


if let Ok((entity, entity_instance, transform)) = new_entity_instances.get_single(){

        if entity_instance.identifier == "Player".to_string() {
            info!("переписываем игрока");

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
                },TextureAtlas {
                    layout,
                    ..default()
                },SpritesheetAnimation::from_id(library.animation_with_name("archer_idle").unwrap()),
                    PlayerBundle{
                        ..default()
                    }
                ))
                .with_children(|commands| {
                    commands.spawn(PlayerChild);
                });
        }
    }
}

pub fn spawn_animations( mut library: ResMut<SpritesheetLibrary>){


    // Create different animations

    // Idle

    let idle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 3).horizontal_strip(0, 0, 3));
    });
    let idle_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(idle_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
    });
    library.name_animation(idle_anim_id, "archer_idle").unwrap();
    // Run Right

    let run_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 3).row(1));
    });

    let run_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(run_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
    });

    library.name_animation(run_anim_id, "run_right").unwrap();




    // Run Left

    let run_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 3).row(2));
    });

    let run_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(run_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
    });

    library.name_animation(run_anim_id, "run_left").unwrap();
}

