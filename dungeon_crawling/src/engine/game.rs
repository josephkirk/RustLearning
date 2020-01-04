use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

pub struct GameMode<'a> {
    pub game_world: &'a mut World,
}

impl<'a> GameMode<'a> {
    pub fn new(world: &mut World) -> GameMode {
        GameMode { game_world: world }
    }

    pub fn initalize_map(&mut self, hole_amount: i32) {
        self.game_world
            .insert(super::Map::new(hole_amount).to_vec());
    }

    pub fn spawn_player(&mut self, x: i32, y: i32) {
        self.game_world
            .create_entity()
            .with(super::Position { x, y })
            .with(super::Renderable {
                glyph: rltk::to_cp437('Ω'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(super::Player {})
            .build();
    }

    pub fn spawn_enemy(&mut self, x: i32, y: i32) {
        self.game_world
            .create_entity()
            .with(super::Position { x, y })
            .with(super::Renderable {
                glyph: rltk::to_cp437('Ö'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(super::Mover { speed: -1 })
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
    let mut positions = ecs.write_storage::<super::Position>();
    let mut players = ecs.write_storage::<super::Player>();
    let map = ecs.fetch::<Vec<super::TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = pos.forecast_idx(delta_x, delta_y);
        // pos.move_relative(x, y);
        match map[destination_idx] {
            super::TileType::Floor => {
                pos.move_relative(delta_x, delta_y);
            }
            _ => {}
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
        let mut mover = super::Mover::init();
        mover.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let map = super::Map(self.ecs.fetch::<Vec<super::TileType>>().to_vec());
        map.draw(ctx);

        self.player_input(ctx);

        self.run_systems();

        let positions = self.ecs.read_storage::<super::Position>();
        let renderables = self.ecs.read_storage::<super::Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
