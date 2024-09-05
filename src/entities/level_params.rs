use bevy::math::{IVec2, Vec2};
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::{GridCoords, LevelIid};

#[derive(Default, Debug, Resource)]
pub struct LevelSizes {
    pub sizes: HashMap<LevelIid, LevelSize>,
}
#[derive(Default, Debug, Resource)]
pub struct LevelCoords {
    pub sizes: HashMap<LevelIid, LevelCoord>,
}

#[derive(Default, Debug)]
pub struct LevelSize {
    pub grid_size: IVec2,
    #[allow(dead_code)]
    pub pix_size: IVec2,
}
#[derive(Default, Debug)]
pub struct LevelCoord {
    pub grid_coords: GridCoords,
    #[allow(dead_code)]
    pub pix_coords: Vec2,
}
