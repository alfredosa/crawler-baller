use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TyleType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TyleType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TyleType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0); // Set the active console to the main console.
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.inside_map(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TyleType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TyleType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                CRIMSON,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }

        // INSTEAD of rendering the whole map... We generate the map (map builder), then
        // get the camera position, which is based on the player position, and then we
        // render only the tiles that are inside the camera view.
        // KEY CONCEPT

        // for y in 0..SCREEN_HEIGHT {
        //     for x in 0..SCREEN_WIDTH {
        //         let idx = map_idx(x, y);
        //         match self.tiles[idx] {
        //             TyleType::Floor => {
        //                 ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
        //             }
        //             TyleType::Wall => {
        //                 ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
        //             }
        //         }
        //     }
        // }
    }

    pub fn inside_map(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.inside_map(point) && self.tiles[map_idx(point.x, point.y)] == TyleType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.inside_map(point) {
            return None;
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
