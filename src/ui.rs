use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*
    ,
};

use crate::MainCamera;
use crate::states::MyAppState;

pub enum WhatAButton {
    Play,
    Settings,
    Github,
    Exit,
    LoadGame,
}

#[derive(Component)]
pub struct ButtonType {
    button_type: WhatAButton,
}

pub struct MyUiPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MyUiPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mouse_scroll,
                button_system
            )
                .run_if(in_state(self.state.clone())),
        )
            // .insert_resource(WinitSettings::desktop_app())
            .add_systems(Startup, setup_ui_camera)

        ;
    }
}

pub fn activate_ui_camera(
    mut camera: Query<&mut Camera, With<IsDefaultUiCamera>>
)
{
    for mut i in camera.iter_mut() {
        i.is_active = true;
    }
}

pub fn deactivate_ui_camera(
    mut camera: Query<&mut Camera, With<IsDefaultUiCamera>>
)
{
    for mut i in camera.iter_mut() {
        i.is_active = false;
    }
}

pub fn activate_main_camera(
    mut camera: Query<&mut Camera, With<MainCamera>>
)
{
    for mut i in camera.iter_mut() {
        i.is_active = true;
    }
}

pub fn deactivate_main_camera(
    mut camera: Query<&mut Camera, With<MainCamera>>
)
{
    for mut i in camera.iter_mut() {
        i.is_active = false;
    }
}

pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

pub fn despawn_ui(mut commands: Commands,
                  query: Query<Entity, With<Node>>) {
    for i in query.iter() {
        commands.entity(i).despawn_recursive();
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            // Center buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        height: Val::Percent(50.0),
                        width: Val::Percent(30.0),
                        ..default()
                    },
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // "Play" button
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center, // Центрирует по горизонтали
                            align_items: AlignItems::Center,         // Центрирует по вертикали
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.7).into(),

                        ..default()
                    }, ButtonType { button_type: WhatAButton::Play }
                    ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            );
                        });

                    // "Load Game" button
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center, // Центрирует по горизонтали
                            align_items: AlignItems::Center,         // Центрирует по вертикали
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.7).into(),
                        ..default()
                    }, ButtonType { button_type: WhatAButton::LoadGame }))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Load Game",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            );
                        });
                    // "Settings" button
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center, // Центрирует по горизонтали
                            align_items: AlignItems::Center,         // Центрирует по вертикали
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.7).into(),
                        ..default()
                    }, ButtonType { button_type: WhatAButton::Settings }))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });


                    // "Exit" button
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center, // Центрирует по горизонтали
                            align_items: AlignItems::Center,         // Центрирует по вертикали
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.7).into(),
                        ..default()
                    }, ButtonType { button_type: WhatAButton::Exit }))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit",
                                TextStyle {
                                    font: asset_server.load("fonts/PIxelpointRegular.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });

            // GitHub button in the bottom-left corner
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center, // Центрирует по горизонтали
                            align_items: AlignItems::Center,         // Центрирует по вертикали
                            ..default()
                        },
                        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                        ..default()
                    }, ButtonType { button_type: WhatAButton::Github }))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("icons/github.png")),
                                background_color: Color::rgb(0., 0., 0.).into(),
                                ..default()
                            });
                        });
                });
        });
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

//TODO переместить эту систему в гейм плагин и менять стейт на Pause
pub fn toggle_pause_game(
    state: Res<State<MyAppState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {

    // if let Some()
    if input.just_pressed(KeyCode::Escape) {
        match state.get() {
            MyAppState::InGame => next_state.set(MyAppState::MainMenu),
            MyAppState::MainMenu => next_state.set(MyAppState::InGame),
            _ => {}
        }
    }
}

// Система для обработки взаимодействий с кнопкой
fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonType),(Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, button_type) in &mut interaction_query {

        match *interaction {
            Interaction::Pressed => {
                match button_type.button_type {
                    WhatAButton::Play => {println!("Play button clicked!");}
                    WhatAButton::Settings => {println!("Settings button clicked!");}
                    WhatAButton::Github => {println!("Github button clicked!");}
                    WhatAButton::Exit => {println!("Exit button clicked!");}
                    WhatAButton::LoadGame => {println!("LoadGame button clicked!");}
                }
                // Обработка события нажатия
                *color = BackgroundColor(Color::rgb(0.7, 0.7, 0.9));

            }
            Interaction::Hovered => {
                // Обработка наведения курсора на кнопку
                *color = BackgroundColor(Color::rgb(0.5, 0.5, 0.9));
            }
            Interaction::None => {
                // Возвращаем цвет к исходному, когда взаимодействие прекращается
                *color = BackgroundColor(Color::rgb(0.3, 0.3, 0.7));
            }
        }
    }
}