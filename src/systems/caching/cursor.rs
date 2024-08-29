use crate::resources::cursor_position::MyWorldCoords;
use crate::MainCamera;
use bevy::prelude::{GlobalTransform, Query, ResMut, Window, With};
use bevy::window::PrimaryWindow;
use bevy_render::prelude::Camera;



pub fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}