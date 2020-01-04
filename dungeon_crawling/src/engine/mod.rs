use rltk::Rltk;
use specs::prelude::*;

mod utils;
pub use utils::*;

mod map;
pub use map::*;

mod components;
pub use components::*;

mod game;
pub use game::*;

// rltk::add_wasm_support!();

pub fn start(window_width: u32, window_height: u32, window_title: &str) {
    // initialize Window
    let context = Rltk::init_simple8x8(window_width, window_height, window_title, "resources");

    // initalize World and register Components
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Mover>();
    gs.ecs.register::<Player>();

    //

    let mut gm = GameMode {
        game_world: &mut gs.ecs,
    };

    let hole_amount = 100;

    gm.initalize_map(hole_amount);

    gm.spawn_player(40, 25);

    // gm.spawn_enemies( 10 );

    rltk::main_loop(context, gs);
}
