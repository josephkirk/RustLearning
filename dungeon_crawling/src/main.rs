use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};


#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate log;

pub static WINDOW_WIDTH: u32 = 80;
pub static WINDOW_HEIGHT: u32 = 50;
pub static WINDOW_TITLE: &'static str = "DUNGEON CRAWLING";

pub static MAX_WIDTH: i32 = WINDOW_WIDTH as i32 - 1;
pub static MAX_HEIGHT: i32 = WINDOW_HEIGHT as i32 - 1;

pub static HALF_WIDTH: i32 = WINDOW_WIDTH as i32 / 2;
pub static HALF_HEIGHT: i32 = WINDOW_HEIGHT as i32 / 2;

#[allow(dead_code)]
mod engine;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{}::{}:: {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub static LOGGER: SimpleLogger = SimpleLogger;

pub fn init_logger() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger)).map(|()| log::set_max_level(LevelFilter::Debug))
}

fn game_logic(_gm: &mut engine::GameMode) {

}

fn main() -> Result<(), SetLoggerError> {
    // initialize Window
    init_logger()?;
    engine::start(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, &game_logic);
    Ok(())
}
