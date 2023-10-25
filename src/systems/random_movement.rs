use std::process::Command;

use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandom)]
/// the random movement system takes care of moving all the entities that have the MovingRandom component AKA enemies
pub fn random_movement(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &mut Point, &MovingRandom)>::query();
    //iter_mut vs iter: iter_mut is Legion syntax for iterating over a query result with param mutability. which means we can mutate the world.
    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, 1),
            _ => Point::new(0, -1),
        } + *pos;

        commands
            .push(((), WantsToMove{entity: *entity, destination}));
    })
}
