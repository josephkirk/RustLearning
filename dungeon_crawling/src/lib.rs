// use std::io;
use std::error::Error;

struct GameComponent {
    id: i32,
    name: String
}

struct GameObject {
    id: i32,
    name: String,
    components: Vec<GameComponent>
}

struct Point<T> {
    x: T,
    y: T
}

struct Transform {
    matrix:
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // panic!("Not Ok!");
    println!("Run Ok!");
    Ok(())
}