//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .insert_resource(MovementY(MovementYCoord::No))
        .insert_resource(MovementX(MovementXCoord::No))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(EditorPlugin::default())
        .add_systems(Startup, (setup_camera, setup_character))
        .add_systems(Update, sprite_movement)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, keyboard_input_system)
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MyGameCamera,
    ));
}
#[derive(Component)]
struct Player;
#[derive(Component)]
struct MyGameCamera;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

#[derive(Component)]
enum Direction {
    Up,
    _Down,
}

#[derive(Clone, Copy, Debug)]
// #[derive(Resource)]
enum MovementYCoord {
    Up,
    Down,
    No,
}

#[derive(Clone, Copy)]
enum MovementXCoord {
    Left,
    Right,
    No,
}

#[derive(Resource)]
struct MovementX(MovementXCoord);

impl MovementX {
    pub fn get(&self) -> MovementXCoord {
        self.0
    }
    pub fn set(&mut self, m: MovementXCoord) {
        self.0 = m;
    }
}

#[derive(Resource)]
struct MovementY(MovementYCoord);

impl crate::MovementY {
    pub fn get(&self) -> MovementYCoord {
        self.0
    }
    pub fn set(&mut self, m: MovementYCoord) {
        self.0 = m;
    }
}

fn setup_character(mut commands: Commands, asset_server: Res<AssetServer>,
                   mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load("goose/universal_man.png");
    let num_of_sprites = 8;
    let num_of_frames = 2;
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), num_of_frames, num_of_sprites, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let transform = Transform { scale: Vec3::splat(3.0), translation: Vec3::new(100., 0., 0.), ..default() };
    let sprite_bundle = SpriteBundle {
        texture,
        transform,
        ..default()
    };

    for i in 0..num_of_sprites {
        if i == 4 || i == 6 {
            continue;
        }
        let animation_indices = AnimationIndices { first: i * num_of_frames, last: (i + 1) * num_of_frames - 1 };
        let k = (
            sprite_bundle.clone(),
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            Direction::Up,
        );
        commands.spawn(k);
    }
}
fn sprite_movement(_time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>, movement_x: Res<MovementX>, movement_y: Res<MovementY>) {
    for (mut _logo, mut transform) in &mut sprite_position {
        match movement_y.get() {
            MovementYCoord::Up => transform.translation.y += 5.,
            MovementYCoord::Down => transform.translation.y -= 5.,// * time.delta_seconds(),
            _ => {}
        }
        match movement_x.get() {
            MovementXCoord::Left => transform.translation.x -= 5.,// * time.delta_seconds(),
            MovementXCoord::Right => transform.translation.x += 5.,// * time.delta_seconds(),
            _ => {}
        }
    }
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut movement_x: ResMut<MovementX>, mut movement_y: ResMut<MovementY>) {
    match (keyboard_input.pressed(KeyCode::KeyS), keyboard_input.pressed(KeyCode::KeyW)) {
        (true, false) => movement_y.set(MovementYCoord::Down),
        (false, true) => movement_y.set(MovementYCoord::Up),
        _ => movement_y.set(MovementYCoord::No),
    }
    match (keyboard_input.pressed(KeyCode::KeyA), keyboard_input.pressed(KeyCode::KeyD)) {
        (true, false) => movement_x.set(MovementXCoord::Left),
        (false, true) => movement_x.set(MovementXCoord::Right),
        _ => movement_x.set(MovementXCoord::No),
    }

    match keyboard_input.just_pressed(KeyCode::Space) {
        true=> {},
        false=> {},
    }
}