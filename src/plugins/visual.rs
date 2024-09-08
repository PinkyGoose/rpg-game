use crate::show_character;
use crate::systems::health::update_health_bars;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, States};

pub struct VisualPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for VisualPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (show_character, update_health_bars).run_if(in_state(self.state.clone())),
        );
    }
}
