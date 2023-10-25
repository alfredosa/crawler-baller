mod camera;
mod components;
mod map;
mod map_builder;
mod spawners;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawners::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_system: Schedule,
    player_system: Schedule,
    enemy_system: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);

        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|p| p.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_system: build_input_scheduler(),
            player_system: build_player_scheduler(),
            enemy_system: build_enemy_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0); // Set the active console to the main console.
        ctx.cls(); // Clear the main console.
        ctx.set_active_console(1); // Set the active console to the UI console.
        ctx.cls(); // Clear the UI console.
        self.resources.insert(ctx.key); // insert the key pressed into the resources (Available for all systems)

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_system
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTurn => {
                self.player_system
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::EnemyTurn => {
                self.enemy_system
                    .execute(&mut self.ecs, &mut self.resources);
            }
        }

        render_draw_buffer(ctx).expect("Render error"); // render the draw buffer (The draw buffer is the console that we are drawing to)
    }
}

fn main() {
    let context = BTermBuilder::new()
        .with_title("Crawler Baller 2D")
        .with_fps_cap(60.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("assets/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()
        .unwrap();

    let _ = main_loop(context, State::new());
}
