use crate::systems::caching::movement::{move_all, move_player_from_input, randomize_movements};
use crate::systems::spawn::check_player_on_entry;
use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, States};

pub struct MovementPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MovementPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_player_from_input,
                move_all,
                randomize_movements,
                check_player_on_entry,
            )
                .run_if(in_state(self.state.clone())),
        );
    }
}
