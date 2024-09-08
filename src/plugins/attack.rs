use crate::systems::actions::attack::{
    attack_player_from_input, check_killed, check_killed_player, move_missiles, randomize_attacks,
};
use crate::systems::caching::attack::insert_enemy_attack_time;
use crate::systems::health::regen_health;
use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, States};

pub struct AttackPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for AttackPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                attack_player_from_input,
                check_killed,
                randomize_attacks,
                move_missiles,
                check_killed_player,
                regen_health,
                insert_enemy_attack_time,
            )
                .run_if(in_state(self.state.clone())),
        );
    }
}
