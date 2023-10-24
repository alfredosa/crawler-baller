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
