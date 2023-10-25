use crate::prelude::*;

#[system]
/// the player input takes care of any action recorded by the player, be it mouse or keys
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
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
            players.iter_mut(ecs).for_each(|(ent, pos)| {
                let destination = *pos + delta;
                commands.push((
                    (),
                    WantsToMove {
                        entity: *ent,
                        destination,
                    },
                ));
            });
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
