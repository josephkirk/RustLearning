use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

use super::{IPosition, Map, Mover, Player, Renderable, TileType};

pub struct GameMode<'a> {
    pub game_world: &'a mut World,
}

impl<'a> GameMode<'a> {
    pub fn new(world: &mut World) -> GameMode {
        GameMode { game_world: world }
    }

    pub fn initalize_map(&mut self, map: Map) {
        self.game_world.insert(map);
    }

    pub fn spawn_player(&mut self, x: i32, y: i32) {
        info!(target: "Game", "Spawn new player at <{}, {}>", x, y);
        self.game_world
            .create_entity()
            .with(IPosition { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('Ω'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player {})
            .build();
    }

    pub fn spawn_enemy(&mut self, x: i32, y: i32) {
        self.game_world
            .create_entity()
            .with(IPosition { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('Ö'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Mover { speed: -1 })
            .build();
    }

    pub fn spawn_enemies(&mut self, amount: i32) {
        for i in 0..amount {
            self.spawn_enemy(i * 7, 14)
        }
    }
}

pub struct State {
    pub ecs: World,
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<IPosition>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = pos.forecast_idx(delta_x, delta_y);
        // pos.move_relative(x, y);
        // debug!(target: "Game", "Attemp to move Player {:?} to <{}, {}>", player, delta_x, delta_y);
        match map.data[destination_idx] {
            TileType::Floor => {
                pos.move_relative(delta_x, delta_y);
                debug!(target: "Game", "move Player {:?} to <{}, {}>", player, pos.x, pos.y);
            }
            _ => {
                debug!(target: "Game", "Collision at <{}, {}>", pos.x+delta_x, pos.y+delta_y);
            }
        }
    }
}

impl State {
    /// Handle Player Movement
    pub fn player_input(&mut self, ctx: &mut Rltk) {
        match ctx.key {
            None => {} // Idle
            Some(key) => match key {
                VirtualKeyCode::Left => try_move_player(-1, 0, &mut self.ecs),
                VirtualKeyCode::Right => try_move_player(1, 0, &mut self.ecs),
                VirtualKeyCode::Up => try_move_player(0, -1, &mut self.ecs),
                VirtualKeyCode::Down => try_move_player(0, 1, &mut self.ecs),
                _ => {} // Idle if other key press
            },
        }
    }

    pub fn run_systems(&mut self) {
        let mut mover = Mover::init();
        mover.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // debug!("{}", map);
        self.player_input(ctx);

        self.run_systems();
        let map = self.ecs.fetch::<Map>();
        map.draw(ctx);

        let positions = self.ecs.read_storage::<IPosition>();
        let renderables = self.ecs.read_storage::<Renderable>();
        // self.ecs.insert(map);
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
