use crate::{camera, prelude::*};

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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
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

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                VirtualKeyCode::Space => Point::new(0, 0),
                _ => Point::new(0, 0),
            };

            // print if key pressed:

            if key == VirtualKeyCode::Left
                || key == VirtualKeyCode::Right
                || key == VirtualKeyCode::Up
                || key == VirtualKeyCode::Down
            {
                println!("Key pressed: {:?}", key);
            }

            if key == VirtualKeyCode::Space {
                self.take_damage(99.0) // change soon to be damage
            }

            if map.can_enter_tile(self.position + delta) {
                self.position += delta;
                camera.on_player_move(self.position);
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
