#![allow(dead_code)]

rltk::add_wasm_support!();
use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
//use std::convert::From;
use std::ops::{Deref, DerefMut};
#[macro_use]
extern crate specs_derive;

const WINDOW_WIDTH: u32 = 80;
const WINDOW_HEIGHT: u32 = 50;
const WINDOW_TITLE: &'static str = "DUNGEON CRAWLING";

const MAX_WIDTH: i32 = WINDOW_WIDTH as i32 - 1;
const MAX_HEIGHT: i32 = WINDOW_HEIGHT as i32 - 1;

const HALF_WIDTH: i32 = WINDOW_WIDTH as i32/2;
const HALF_HEIGHT: i32 = WINDOW_HEIGHT as i32/2;



#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall, Floor, Hole
}


pub fn xy_idx(x: i32, y: i32) -> usize {
    ((y * WINDOW_WIDTH as i32) + x) as usize
}

// #[derive(PartialEq, Copy, Clone)]
struct Map(Vec<TileType>);

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
    fn new(hole_amount: i32) -> Map {
        let mut map = Map(vec![TileType::Floor; (WINDOW_WIDTH*WINDOW_HEIGHT) as usize]);

        for x in 0..MAX_WIDTH {
            map[xy_idx( x, 0 )] = TileType::Wall;
            map[xy_idx( x, MAX_HEIGHT )] = TileType::Wall;
        }

        for y in 0..MAX_HEIGHT {
            map[xy_idx( 0, y)] = TileType::Wall;
            map[xy_idx( MAX_WIDTH, y)] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..hole_amount {
            let x = rng.roll_dice(1, MAX_WIDTH-1);
            let y = rng.roll_dice(1, MAX_HEIGHT-1);
            let idx = xy_idx(x, y);
            if idx != xy_idx(HALF_WIDTH, HALF_HEIGHT) {
                map[idx] = TileType::Hole;
            }
        }
        map
    }

    fn draw(&self, ctx: &mut Rltk) {
        let mut y = 0;
        let mut x = 0;
        for tile in self.iter() {
            match tile {
                TileType::Floor => {
                    ctx.set(
                        x, y,
                        RGB::from_f32(0.25, 0.25, 0.25),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('≡')
                    )
                },
                TileType::Wall => {
                    ctx.set(
                        x, y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('║')
                    )
                },
                TileType::Hole => {
                    ctx.set(
                        x, y,
                        RGB::from_f32(0., 0., 0.),
                        RGB::from_f32(0., 0., 0.),
                        rltk::to_cp437('■')
                    )
                },
            }

            x += 1;
            if x > MAX_WIDTH {
                x = 0;
                y += 1;
            }
        }
    }
}

#[derive(Component)]
struct Position {
    x:i32,
    y:i32,
}

impl Position {
    fn to_idx(&self) -> usize {
        xy_idx(self.x, self.y)
    }

    fn forecast_idx(&self, x: i32, y:i32) -> usize {
        xy_idx(self.x + x, self.y + y)
    }

    fn move_relative(&mut self, x: i32, y: i32) {
        self.x = min(MAX_WIDTH , max(0, self.x + x));
        self.y = min(MAX_HEIGHT, max(0, self.y + y));
    }
}


/*
if not using #[derive(Component)] from specs_derive
impl Component for Position {
    type Storage = VecStorage<Self>;
}
*/

#[derive(Component)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Mover {
    speed: i32
}

impl Mover {
    fn init() -> Mover {
        Mover { speed:1 }
    }
}

impl <'a> System<'a> for Mover {
    type SystemData = (ReadStorage<'a, Mover>,
                       WriteStorage<'a, Position>);
    fn run(&mut self, (mover, mut pos) : Self::SystemData) {
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

#[derive(Component)]
struct Player { }

impl Player {

    
    fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
        let mut positions = ecs.write_storage::<Position>();
        let mut players = ecs.write_storage::<Player>();
        let map = ecs.fetch::<Vec<TileType>>();
        
        for (_player, pos) in (&mut players, &mut positions).join() {
            let destination_idx = pos.forecast_idx( delta_x, delta_y);
            // pos.move_relative(x, y);
            match map[destination_idx] {
                TileType::Floor => {
                    pos.move_relative(delta_x, delta_y);
                },
                _ => {},
            }
        }
    }
}

struct GameMode<'a> {
    game_world: &'a mut World
}

impl <'a>GameMode<'a> {
    fn new(world: &mut World) -> GameMode {
        GameMode {game_world: world}
    }

    fn initalize_map(&mut self, hole_amount: i32) {
        self.game_world.insert(Map::new(hole_amount).to_vec());
    }
    
    fn spawn_player( &mut self, x: i32, y: i32) {
        self.game_world
            .create_entity()
            .with(Position {x, y})
            .with(Renderable {
                glyph: rltk::to_cp437('Ω'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .build();
    }

    fn spawn_enemy( &mut self, x: i32, y: i32) {
        self.game_world
            .create_entity()
            .with(Position {x, y})
            .with(Renderable {
                glyph: rltk::to_cp437('Ö'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Mover {speed: -1})
            .build();
    }
    
    fn spawn_enemies( &mut self, amount: i32) {
        for i in 0..amount {
            self.spawn_enemy( i*7, 14)
        }
    }
}

struct State {
    ecs: World,
}

impl State {
    fn player_input(&mut self, ctx: &mut Rltk) {
        // Player Movement
        match ctx.key {
            None => {} // Idle
            Some(key) => match key {
                VirtualKeyCode::Left => Player::try_move_player(-1, 0, &mut self.ecs),
                VirtualKeyCode::Right => Player::try_move_player(1, 0, &mut self.ecs),
                VirtualKeyCode::Up => Player::try_move_player(0, -1, &mut self.ecs),
                VirtualKeyCode::Down => Player::try_move_player(0, 1, &mut self.ecs),
                _ => {} // Idle if other key press
            }
        }
    }

    fn run_systems(&mut self) {
        let mut mover = Mover::init();
        mover.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let map = Map(self.ecs.fetch::<Vec<TileType>>().to_vec());
        map.draw(ctx);

        self.player_input(ctx);
        
        self.run_systems();
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
        

    }
}

fn main() {

    // initialize Window
    let context = Rltk::init_simple8x8(
        WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, "resources"
    );

    // initalize World and register Components
    let mut gs = State{ 
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Mover>();
    gs.ecs.register::<Player>();

    //

    let mut gm = GameMode { game_world: &mut gs.ecs};
    
    let hole_amount = 100;

    gm.initalize_map(hole_amount);    

    gm.spawn_player(40, 25);

    // gm.spawn_enemies( 10 );

    rltk::main_loop(context, gs);
}