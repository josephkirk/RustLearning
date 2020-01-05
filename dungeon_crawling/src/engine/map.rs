rltk::add_wasm_support!();
use rltk::{Console, Rltk, RGB};
//use std::convert::From;
use std::ops::{Deref, DerefMut};

#[allow(unused)]
use crate::{HALF_HEIGHT, HALF_WIDTH, MAX_HEIGHT, MAX_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

#[allow(unused)]
use super::{xy_idx, IPosition, IRect};

pub static MAP_RECT: IRect = IRect {
    x_min: 0,
    y_min: 0,
    x_max: MAX_WIDTH,
    y_max: MAX_HEIGHT,
};
pub static INNERMAP_RECT: IRect = IRect {
    x_min: 1,
    y_min: 1,
    x_max: MAX_WIDTH - 1,
    y_max: MAX_HEIGHT - 1,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Hole,
    Door,
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
    fn new() -> Map {
        Map(vec![
            TileType::Wall;
            (WINDOW_WIDTH * WINDOW_HEIGHT) as usize
        ])
    }

    pub fn generate_empty_map() -> Map {
        info!(target: "Game", "Generate Empty Level");
        Map::new()
    }

    pub fn generate_random_map() -> Map {
        info!(target: "Game", "Generate Random Level");
        let mut map = Map::new();

        //map.add_room_from_pos_center(&IPosition{x: HALF_WIDTH, y: HALF_HEIGHT}, 4, 4);
        // let mut room1 = IRect{x_min: HALF_WIDTH-2, y_min: HALF_WIDTH-2, x_max: HALF_WIDTH+2, y_max: HALF_WIDTH+2};
        let mut rng = rltk::RandomNumberGenerator::new();
        let mut rooms = Vec::new();
        let room1 = map.add_room_from_pos_center(
            &IPosition {
                x: HALF_WIDTH,
                y: HALF_HEIGHT,
            },
            6,
            6,
        );
        rooms.push(room1);
        for _i in 0..10000 {
            let x = rng.roll_dice(1, MAX_WIDTH);
            let y = rng.roll_dice(1, MAX_HEIGHT);
            let random_pos = IPosition { x, y };
            let mut random_room = IRect::new_from_center_position(&random_pos, 6, 6);
            random_room.constraint_to(&INNERMAP_RECT);
            debug!(target: "Game", "Attemp Adding <6x6> Room at <{}, {}>", random_pos.x, random_pos.y);
            let mut is_intersect = false;
            for room in rooms.iter() {
                if random_room.is_intersect(&room) {
                    is_intersect = true;
                    debug!(target: "Game", "Intersecting with {:?}.Abort", room);
                    break;
                }
            }
            if is_intersect {
                continue;
            }
            map.add_room_from_rect(&random_room);
            rooms.push(random_room);
        }
        map
    }

    pub fn clear(&mut self) {
        for (x, y) in MAP_RECT.iter(1) {
            self.set_wall(&IPosition { x, y });
        }
    }

    pub fn add_room_from_pos_center(&mut self, pos: &IPosition, width: i32, height: i32) -> IRect {
        let mut new_rect = IRect::new_from_center_position(pos, width, height);
        new_rect.constraint_to(&INNERMAP_RECT);
        debug!(target: "Game", "Add Room at <{}, {}>", pos.x, pos.y);
        self.add_room_from_rect(&mut new_rect);
        new_rect
    }

    pub fn add_room_from_rect(&mut self, rect: &IRect) {
        // let mut new_rect = IRect::new();
        // new_rect.fit_to(rect);
        // new_rect.constraint_to(&INNERMAP_RECT);
        debug!(target: "Game", "Add <{}x{}> Room with bounding {:?}", rect.width(), rect.height(), rect);

        for (x, y) in rect.iter(1) {
            self.set_floor(&IPosition { x, y });
        }
    }

    pub fn set_tile(&mut self, pos: &IPosition, tile_type: TileType) {
        // debug!(target: "Game", "Set Map Tile at <{}, {}> to {:?}", pos.x, pos.y, tile_type);
        self[pos.to_idx()] = tile_type;
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
        let mut y = 0;
        let mut x = 0;
        for tile in self.iter() {
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
            if x > MAX_WIDTH {
                x = 0;
                y += 1;
            }
        }
    }
}
