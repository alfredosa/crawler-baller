mod player_input;

use crate::prelude::*;
use player_input::player_input_system;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input_system())
        // .add_system(mob_ai_system())
        // .add_system(map_render_system())
        // .add_system(render_system())
        .build()
}
