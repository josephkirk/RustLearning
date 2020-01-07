rltk::add_wasm_support!();
use rltk::{Console, Rltk, RGB};
//use std::convert::From;
use std::ops::{Deref, DerefMut};

#[allow(unused)]
use super::{xy_idx, IPosition, IRect};

#[cfg(debug)]
const ROOM_AMOUNT: i32 = 100;

#[cfg(not(debug))]
const ROOM_AMOUNT: i32 = 1000;

const ROOM_MIN_SIZE: i32 = 6;
const ROOM_MAX_SIZE: i32 = 10;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Hole,
    Door,
}

#[derive(Debug)]
pub struct Map {
    pub data: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<IRect>,
}

// Wrap Map Struct to have the same behavior as Vec

// impl From<Vec<TileType>> for Map {
//     fn from(map: Vec<TileType>) -> Self {
//         Map(

//             data:map)
//     }
// }
// impl Into<Vec<TileType>> for Map {
//     fn into(self) -> Vec<TileType> {
//         self.data
//     }
// }

// impl Deref for Map {
//     type Target = Vec<TileType>;
//     fn deref(&self) -> &Vec<TileType> {
//         &self.data
//     }
// }

// impl DerefMut for Map {
//     fn deref_mut(&mut self) -> &mut Vec<TileType> {
//         &mut self.data
//     }
// }

//

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let data = vec![TileType::Wall; (width * height) as usize];
        let rooms = Vec::new();
        Map {
            width,
            height,
            data,
            rooms,
        }
    }

    pub fn generate_random_map(&mut self) {
        info!(target: "Game", "Generate Random Level");

        //map.build_room_from_center(&IPosition{x: HALF_WIDTH, y: HALF_HEIGHT}, 4, 4);
        // let mut room1 = IRect{x_min: HALF_WIDTH-2, y_min: HALF_WIDTH-2, x_max: HALF_WIDTH+2, y_max: HALF_WIDTH+2};
        let mut rng = rltk::RandomNumberGenerator::new();
        for _i in 0..ROOM_AMOUNT {
            let w = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let h = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
            let x = rng.roll_dice(1, self.width - w - 1);
            let y = rng.roll_dice(1, self.height - h - 1);
            let random_pos = IPosition { x, y };
            let mut random_room = IRect::new_from_center_position(&random_pos, w, h);
            random_room.constraint_to(&self.shrink_map_rect(1));
            debug!(target: "Game", "Attemp Adding <6x6> Room at <{}, {}>", random_pos.x, random_pos.y);
            let mut is_intersect = false;
            for room in self.rooms.iter() {
                if random_room.is_intersect(&room.expand_shrink_as(1)) {
                    is_intersect = true;
                    debug!(target: "Game", "Intersecting with {:?}.Abort", room);
                    break;
                }
            }
            if !is_intersect {
                self.build_room(random_room);
            }
        }
    }

    pub fn map_rect(&self) -> IRect {
        IRect {
            x_min: 0,
            y_min: 0,
            x_max: self.width,
            y_max: self.height,
        }
    }

    pub fn shrink_map_rect(&self, shrink_amnt: i32) -> IRect {
        self.map_rect().expand_shrink_as(-shrink_amnt)
    }

    pub fn clear(&mut self) {
        for (x, y) in self.map_rect().iter(1) {
            self.set_wall(&IPosition { x, y });
        }
    }

    pub fn build_room_from_center(&mut self, pos: &IPosition, width: i32, height: i32) {
        let mut new_rect = IRect::new_from_center_position(pos, width, height);
        new_rect.constraint_to(&self.shrink_map_rect(1));
        debug!(target: "Game", "Add Room at <{}, {}>", pos.x, pos.y);
        self.build_room(new_rect);
    }

    pub fn build_room(&mut self, rect: IRect) {
        // let mut new_rect = IRect::new();
        // new_rect.fit_to(rect);
        // new_rect.constraint_to(&INNERMAP_RECT);
        debug!(target: "Game", "Add <{}x{}> Room with bounding {:?}", rect.width(), rect.height(), rect);

        for (x, y) in rect.iter(1) {
            self.set_floor(&IPosition { x, y });
        }
        self.rooms.push(rect)
    }

    pub fn set_tile(&mut self, pos: &IPosition, tile_type: TileType) {
        // debug!(target: "Game", "Set Map Tile at <{}, {}> to {:?}", pos.x, pos.y, tile_type);
        self.data[pos.to_idx()] = tile_type;
    }

    pub fn set_door(&mut self, pos: &IPosition) {
        self.set_tile(pos, TileType::Door);
    }

    pub fn set_floor(&mut self, pos: &IPosition) {
        self.set_tile(pos, TileType::Floor);
    }

    pub fn set_hole(&mut self, pos: &IPosition) {
        self.set_tile(pos, TileType::Hole);
    }

    pub fn set_wall(&mut self, pos: &IPosition) {
        self.set_tile(pos, TileType::Wall);
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        // debug!("{:?}", self);
        let mut y = 0;
        let mut x = 0;
        for tile in self.data.iter() {
            match tile {
                TileType::Floor => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.1, 0.1, 0.1),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('≡'),
                ),
                TileType::Wall => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.25, 0.25, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('█'),
                ),
                TileType::Hole => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 0., 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('■'),
                ),
                TileType::Door => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 0.5, 0.6),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('∩'),
                ),
            }

            x += 1;
            // debug!("{:?}",ctx.);
            if x > self.width - 1 {
                x = 0;
                y += 1;
            }
        }
    }
}
