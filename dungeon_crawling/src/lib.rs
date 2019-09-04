// use std::io;
use std::error::Error;

mod game;

use crate::game::core;

pub fn run() -> Result<(), Box<dyn Error>> {
    // panic!("Not Ok!");
    println!("Run Ok!");
    Ok(())
}