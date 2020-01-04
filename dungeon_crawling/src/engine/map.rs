rltk::add_wasm_support!();
use rltk::{Console, Rltk, RGB};
//use std::convert::From;
use std::ops::{Deref, DerefMut};

use crate::{HALF_HEIGHT, HALF_WIDTH, MAX_HEIGHT, MAX_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Hole,
}

// #[derive(PartialEq, Copy, Clone)]
pub struct Map(pub Vec<TileType>);

// Wrap Map Struct to have the same behavior as Vec

impl From<Vec<TileType>> for Map {
    fn from(map: Vec<TileType>) -> Self {
        Map(map)
    }
}
impl Into<Vec<TileType>> for Map {
    fn into(self) -> Vec<TileType> {
        self.0
    }
}

impl Deref for Map {
    type Target = Vec<TileType>;
    fn deref(&self) -> &Vec<TileType> {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Vec<TileType> {
        &mut self.0
    }
}

//

impl Map {
    pub fn new(hole_amount: i32) -> Map {
        let mut map = Map(vec![
            TileType::Floor;
            (WINDOW_WIDTH * WINDOW_HEIGHT) as usize
        ]);

        for x in 0..MAX_WIDTH {
            map[super::xy_idx(x, 0)] = TileType::Wall;
            map[super::xy_idx(x, MAX_HEIGHT)] = TileType::Wall;
        }

        for y in 0..MAX_HEIGHT {
            map[super::xy_idx(0, y)] = TileType::Wall;
            map[super::xy_idx(MAX_WIDTH, y)] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..hole_amount {
            let x = rng.roll_dice(1, MAX_WIDTH - 1);
            let y = rng.roll_dice(1, MAX_HEIGHT - 1);
            let idx = super::xy_idx(x, y);
            if idx != super::xy_idx(HALF_WIDTH, HALF_HEIGHT) {
                map[idx] = TileType::Hole;
            }
        }
        map
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let mut y = 0;
        let mut x = 0;
        for tile in self.iter() {
            match tile {
                TileType::Floor => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.25, 0.25, 0.25),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('≡'),
                ),
                TileType::Wall => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('║'),
                ),
                TileType::Hole => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 0., 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('■'),
                ),
            }

            x += 1;
            if x > MAX_WIDTH {
                x = 0;
                y += 1;
            }
        }
    }
}
