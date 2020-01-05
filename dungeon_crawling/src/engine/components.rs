use rltk::RGB;
use specs::prelude::*;
use std::cmp::{max, min};

use crate::{MAX_HEIGHT, MAX_WIDTH};

use super::{xy_idx};

#[derive(Component, Debug, PartialEq, Copy, Clone)]
pub struct IPosition {
    pub x: i32,
    pub y: i32,
}

pub struct Position {
    pub x: f64,
    pub y: f64,

}

impl IPosition {
    pub fn to_idx(&self) -> usize {
        xy_idx(self.x, self.y)
    }

    pub fn forecast_idx(&self, x: i32, y: i32) -> usize {
        xy_idx(self.x + x, self.y + y)
    }

    pub fn move_relative(&mut self, x: i32, y: i32) {
        self.x = min(MAX_WIDTH, max(0, self.x + x));
        self.y = min(MAX_HEIGHT, max(0, self.y + y));
    }
}

/*
implementation Component if not using #[derive(Component)] from specs_derive
impl Component for IPosition {
    type Storage = VecStorage<Self>;
}
*/

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Mover {
    pub speed: i32,
}

impl Mover {
    pub fn init() -> Mover {
        Mover { speed: 1 }
    }
}

impl<'a> System<'a> for Mover {
    type SystemData = (ReadStorage<'a, Mover>, WriteStorage<'a, IPosition>);
    fn run(&mut self, (mover, mut pos): Self::SystemData) {
        for (mover, pos) in (&mover, &mut pos).join() {
            pos.x -= mover.speed;
            if pos.x > MAX_WIDTH {
                pos.x = 0;
            } else if pos.x < 0 {
                pos.x = MAX_WIDTH;
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Player {}

// impl Player {

// }
