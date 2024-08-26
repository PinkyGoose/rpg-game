use bevy::math::Vec3;
use bevy::prelude::{Added, BuildChildren, Commands, default, Entity, info, Query, Res, Sprite, Time, Transform, Vec2};
use bevy::sprite::{Anchor, SpriteBundle};
use bevy_color::Color;
use bevy_color::palettes::basic::{GRAY, GREEN, RED};
use crate::entities::health::{Health, HealthBar, HealthBarBackground, Regeneration};
use crate::entities::player::Player;

// Система для создания полоски здоровья
pub fn spawn_health_bars(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Player>), Added<Health>>,
) {
    for (entity, health, player) in query.iter() {
        // Спавним фоновую полоску здоровья
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from(GRAY),
                        custom_size: Some(Vec2::new(50.0, 5.0)), // Размер фона полоски здоровья
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 40.0, 0.0)), // Положение фона полоски здоровья над игроком
                    ..default()
                },
                HealthBarBackground,
            ));

            // Спавним основную полоску здоровья
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from(match player {
                            None => RED,
                            Some(_) => GREEN
                        }),
                        custom_size: Some(Vec2::new(
                            50.0 * (health.current / health.max),
                            5.0,
                        )), // Длина полоски здоровья в зависимости от текущего значения
                        anchor: Anchor::CenterLeft, // Привязка к левому краю
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(-25.0, 40.0, 1.0)), // Положение полоски здоровья над игроком (смещение по оси X)
                    ..default()
                },
                HealthBar
            ));
        });
    }
}

pub fn regen_health(
    time: Res<Time>,
    mut query: Query<(&mut Health, &Regeneration)>
){

    for (mut health, regen) in query.iter_mut() {
        info! ("regen_health");
        let mut new_health = health.current+ regen.0 * time.delta_seconds();
        // info!("{destination:?}");
        if new_health >health.max {
            new_health = health.max;
        }
            health.current = new_health;

    }
}
