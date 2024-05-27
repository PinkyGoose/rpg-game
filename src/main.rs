//! Renders a 2D scene containing a single, moving sprite.

use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;
use bevy::window::WindowResized;

fn main() {
    App::new()
        .insert_resource(Movement_Y(MovementY::No))
        .insert_resource(Movement_X(MovementX::No))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup, setup_character))
        .add_systems(Update, sprite_movement)
        .add_systems(Update, animate_sprite)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, get_window)
        .run();
}

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
    Down,
}

#[derive(Clone, Copy, Debug)]
// #[derive(Resource)]
enum MovementY {
    Up,
    Down,
    No,
}

#[derive(Clone, Copy)]
enum MovementX {
    Left,
    Right,
    No,
}

#[derive(Resource)]
struct Movement_X(MovementX);

impl Movement_X {
    pub fn get(&self) -> MovementX {
        self.0
    }
    pub fn set(&mut self, m: MovementX) {
        self.0 = m;
    }
}

#[derive(Resource)]
struct Movement_Y(MovementY);

impl crate::Movement_Y {
    pub fn get(&self) -> MovementY {
        self.0
    }
    pub fn set(&mut self, m: MovementY) {
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,
         mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    commands.spawn(Camera2dBundle::default());

    setup_character(commands, asset_server, texture_atlas_layouts);
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>, movement_x: Res<Movement_X>, movement_y: Res<Movement_Y>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match movement_y.get() {
            MovementY::Up => transform.translation.y += 5.,
            MovementY::Down => transform.translation.y -= 5.,// * time.delta_seconds(),
            _ => {}
        }
        match movement_x.get() {
            MovementX::Left => transform.translation.x -= 5.,// * time.delta_seconds(),
            MovementX::Right => transform.translation.x += 5.,// * time.delta_seconds(),
            _ => {}
        }
    }
}

fn get_window(window: Query<&Window>) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let (x, y) = match window.position {
        WindowPosition::At(v) => (v.x as f32, v.y as f32),
        _ => (0., 0.),
    };

    dbg!(width, height, x, y);
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>, mut movement_x: ResMut<Movement_X>, mut movement_y: ResMut<Movement_Y>) {
    match (keyboard_input.pressed(KeyCode::KeyS), keyboard_input.pressed(KeyCode::KeyW)) {
        (true, false) => movement_y.set(MovementY::Down),
        (false, true) => movement_y.set(MovementY::Up),
        _ => movement_y.set(MovementY::No),
    }
    match (keyboard_input.pressed(KeyCode::KeyA), keyboard_input.pressed(KeyCode::KeyD)) {
        (true, false) => movement_x.set(MovementX::Left),
        (false, true) => movement_x.set(MovementX::Right),
        _ => movement_x.set(MovementX::No),
    }
}