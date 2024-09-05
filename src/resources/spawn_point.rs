use bevy::prelude::ReflectResource;
use bevy::prelude::{Deref, DerefMut, Reflect, Resource};
use bevy_ecs_ldtk::EntityIid;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
#[derive(Debug, Deref, DerefMut, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SpawnPointId(pub Option<EntityIid>);
