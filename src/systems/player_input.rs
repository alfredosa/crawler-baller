use crate::prelude::*;

#[system]
/// the player input takes care of any action recorded by the player, be it mouse or keys
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::Space => Point::new(0, 0),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            // Get the player position from the world. (Query the world for the player position) &mut Point is a mutable reference to the player position.
            // How the Query works: We are querying the world for all entities that have a Point component and a Player component.
            let mut players = <&mut Point>::query().filter(component::<Player>());
            // ecs in iter_mut is a mutable reference to the world. We are iterating over all the players in the world.
            // iter mut is Legion syntax for iterating over a query result with param mutability.
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}
