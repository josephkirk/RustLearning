use rltk::Rltk;
use specs::prelude::*;

mod utils;
pub use utils::*;

mod drawing;
pub use drawing::*;

mod map;
pub use map::*;

mod components;
pub use components::*;

mod game;
pub use game::*;

// rltk::add_wasm_support!();

pub fn start(window_width: u32, window_height: u32, window_title: &str) {
    // initialize Window
    info!(target: "Game", "Initialize {}x{} Window with title {} ", window_width, window_height, window_title);
    let context = Rltk::init_simple8x8(window_width, window_height, window_title, "resources");

    // initalize World and register Components
    info!(target: "Game", "Initialize Game State");
    let mut gs = State { ecs: World::new() };


    info!(target: "Game", "Register Game Components");
    gs.ecs.register::<IPosition>(); 
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Mover>();
    gs.ecs.register::<Player>();

    info!(target: "Game", "Register Game Mode");
    let mut gm = GameMode {
        game_world: &mut gs.ecs,
    };

    info!(target: "Game", "Setup Level");
    let level = Map::generate_random_map();
    gm.initalize_map(level);

    info!(target: "Game", "Spawn Player");
    gm.spawn_player(window_width as i32 /2, window_height as i32/2);

    // gm.spawn_enemies( 10 );

    rltk::main_loop(context, gs);
}
