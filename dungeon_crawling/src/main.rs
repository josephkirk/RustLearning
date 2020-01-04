#[macro_use]
extern crate specs_derive;

pub static WINDOW_WIDTH: u32 = 50;
pub static WINDOW_HEIGHT: u32 = 50;
pub static WINDOW_TITLE: &'static str = "DUNGEON CRAWLING";

pub static MAX_WIDTH: i32 = WINDOW_WIDTH as i32 - 1;
pub static MAX_HEIGHT: i32 = WINDOW_HEIGHT as i32 - 1;

pub static HALF_WIDTH: i32 = WINDOW_WIDTH as i32 / 2;
pub static HALF_HEIGHT: i32 = WINDOW_HEIGHT as i32 / 2;

#[allow(dead_code)]
mod engine;

fn main() {
    // initialize Window
    engine::start(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
}
