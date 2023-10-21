use crate::prelude::*;

pub struct Player {
    pub health: f32,
    pub position: Point,
    pub xp: f32,
    pub level: i32,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            health: 100.,
            position,
            xp: 0.0,
            level: 1,
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn level_up(&mut self) {
        if self.xp > 1.0 {
            self.xp -= 1.0;
            self.level += 1;
        }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Space => Point::new(0, 0),
                _ => Point::new(0, 0),
            };

            if key == VirtualKeyCode::Space {
                self.take_damage(99.0) // change soon to be damage
            }

            if map.can_enter_tile(self.position + delta) {
                self.position += delta;
            }
        }
    }

    pub fn add_xp(&mut self, xp: f32) {
        self.xp += xp;
        self.level_up();
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health < 1.0 {
            println!("You are dead");
            todo!("Game over game state swap");
        }
    }
}
