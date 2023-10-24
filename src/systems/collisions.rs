use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Point)]
pub fn collisions(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut player_position = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_position = *pos);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_position)
        .for_each(|(entity, _)| commands.remove(*entity));
}
