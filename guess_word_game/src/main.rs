use std::io;
use rand::Rng;

fn pick_random(slice: &[&str]) -> String {
    let rand_num = rand::thread_rng().gen_range(0, slice.len()-1);
    slice[rand_num].to_string()
}

fn main() {
    let animal_names = [
        "cow", "cat", "dog", "elephant", "snake",
        "wolverine", "ratel", "lion", "tiger"
    ];
    println!("Guess the Animal Game !");
    println!("Animal list: {:?}", animal_names);
    let secret_animal = pick_random(&animal_names);

    println!("Pick {} as secret animal", secret_animal);
    println!("Type-in your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read input!");
    
    println!("You guess {}", guess);
}
