// Author: Nguyen Phi Hung
// App: Guess the animal name game

use std::io;
use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("Guess the Animal Game !");

    let animal_names = [
        "cow", "cat", "dog", "elephant", "snake",
        "wolverine", "ratel", "lion", "tiger"
    ];
    
    // Debug
    // println!("Animal list: {:?}", animal_names);
    loop {
        let secret_animal = pick_random(&animal_names);
        println!("Generate secret animal name...");

        // Debug
        // println!("Pick {} as secret animal", secret_animal);

        println!("Type-in your guess:");

        let mut guess = String::new();
        read_console_input(&mut guess);

        println!("You guess {}", guess);

        check_guess(&guess, &secret_animal);
    }
}

fn read_console_input(mut input_str: &mut String) {
    io::stdin().read_line(&mut input_str)
        .expect("Failed to read console_input!");
    let trim_input = String::from(input_str.trim());
    input_str.clear();
    input_str.push_str(&trim_input);
}

fn pick_random<'a>(slice: &'a [&str]) -> &'a str {
    let rand_num = rand::thread_rng().gen_range(0, slice.len()-1);
    slice[rand_num]
}

fn check_guess(guess: &str, result:&str) {
    if guess==result {
        println!("*********Congratulation! The secret animal is the {}!*********", result)
    } else {
        println!("Your guess is wrong!");
        let char_matched: Vec<char> = extract_matching_char(&guess, &result);

        // Debug
        // println!("{:?}", char_matched);
        
        println!("There is {} characters in your guess that match the secret animal!", char_matched.len());
    }
}

fn extract_matching_char(str1: &str, str2: &str) -> Vec<char> {
    let mut matched_chars = Vec::new();
    for str1_char in str1.chars() {
        for str2_char in str2.chars() {
            if str1_char == str2_char {
                matched_chars.push(str1_char)
            }
        }
    }
    matched_chars.dedup();
    matched_chars
}