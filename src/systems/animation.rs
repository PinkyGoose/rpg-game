use bevy::prelude::ResMut;
use bevy_spritesheet_animation::library::SpritesheetLibrary;
use bevy_spritesheet_animation::prelude::{AnimationRepeat, Spritesheet};

pub fn spawn_animations(mut library: ResMut<SpritesheetLibrary>) {


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

