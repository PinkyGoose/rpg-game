use crate::systems::caching::coords::translate_grid_coords_entities;
use crate::systems::caching::cursor::my_cursor_system;
use crate::systems::caching::friendly::calculate_friendly;
use crate::systems::caching::level_params::cache_level_params;
use crate::systems::caching::visible_distanse::calculate_visible;
use crate::systems::caching::wall::cache_wall_locations;
use crate::systems::health::{calculate_health, spawn_health_bars};
use crate::systems::spawn::cache_neighbor_levels;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, States};

pub struct CachePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for CachePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                translate_grid_coords_entities,
                cache_wall_locations.after(cache_neighbor_levels),
                cache_neighbor_levels.after(cache_level_params),
                cache_level_params,
                spawn_health_bars,
                calculate_friendly,
                calculate_health,
                calculate_visible,
                my_cursor_system,
            )
                .run_if(in_state(self.state.clone())),
        );
    }
}
