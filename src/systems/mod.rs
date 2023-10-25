mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod movement;
mod player_input;
mod random_movement;

use crate::prelude::*;

/// takes the input of the player at any given moment
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

/// takes care of mounting all the systems and building them on player turn
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

/// takes care of mounting all systems and building them for the enemy turn

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_movement::random_movement_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
