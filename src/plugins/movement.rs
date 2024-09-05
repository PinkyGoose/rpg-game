use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, States};
use crate::systems::caching::movement::move_player_from_input;

pub struct MyPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MyPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_player_from_input,
            // my_plugin_system2,
            // ...
        ).run_if(in_state(self.state.clone())));
    }
}