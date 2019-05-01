// Author: Nguyen Phi Hung
// App: Guess the animal name game
// TODO: Check if the guess is nonsence word
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
// extern crate serde;
// extern crate serde_json;
// extern crate env_logger;
// extern crate regex;
// extern crate colored;

use crossterm::{Crossterm, ClearType, Colored, Color, Colorize, Styler, Attribute};
use std::io;
use rand::Rng;
use regex::Regex;
use std::{thread, time};
// use std::cmp::Ordering;

// Define Enum
enum Guess {
    Right,
    Wrong,
    Invalid
}

// Define Constant
// const ONE_SEC: time::Duration = time::Duration::from_secs(1);
// const TEN_MILIS: time::Duration = time::Duration::from_millis(100);

// Define Struct
#[derive(Debug, Serialize, Deserialize)]
struct Animal {
    name: String,
    r#type: String, // r# to avoid calling standard method instead of define.
    features: String,
}

// Define Functions

fn read_console_input(mut input_str: &mut String) {
    io::stdin().read_line(&mut input_str)
        .expect("Failed to read console_input!");
    let trim_input = String::from(input_str.trim());
    input_str.clear();
    input_str.push_str(&trim_input);
}


fn pick_random_index(slice_len: usize) -> usize {
    rand::thread_rng().gen_range(0, slice_len-1)
}

fn check_guess(guess: &str, result_validator:&Regex, input_validator:&Regex, thinking_duration: time::Duration) -> Guess {
    thread::sleep(thinking_duration);
    println!(".");
    thread::sleep(thinking_duration);
    if !input_validator.is_match(&guess) {
        return Guess::Invalid
    }
    println!("..");
    thread::sleep(thinking_duration);
    if !result_validator.is_match(&guess) {
        return Guess::Wrong
    }
    println!("...");
    thread::sleep(thinking_duration);
    Guess::Right
}

fn extract_matching_char(str1: &str, str2: &str) -> Vec<(usize, char)> {
    let mut matched_chars = Vec::new();
    for (char_index, (str1_char, str2_char)) in str1.chars().zip(str2.chars()).enumerate() {
        if str1_char.eq_ignore_ascii_case(&str2_char) {
            matched_chars.push((char_index, str1_char))
        }
    }
    // matched_chars.dedup(); // remove element duplication
    matched_chars
}

// Main

fn main() {
    let cterm = Crossterm::new();
    let terminal = cterm.terminal();
    let term_color = cterm.color();
    terminal.clear(ClearType::All).unwrap();
    env_logger::init();
    let animal_data_string = include_str!("animal_datas.json");
    let animal_data: Vec<Animal> = serde_json::from_str(&animal_data_string).unwrap_or_else(|error| {
        panic!("Something wrong when parsing data {:?}", error)
    });

    let mut guess_count: i64;
    let anticipate_time = time::Duration::from_millis(500);
    let input_validator = Regex::new(r"^([A-Za-z]+\s*)+$").unwrap();
    debug!("Animal Database: {:?}", animal_data);
    debug!("Result anticipate duration: {:?}", anticipate_time);

    println!("{}", "--------------------------------".yellow().on_magenta().negative().rapid_blink());
    println!("{}", "-----Guess the Animal Game------".yellow().on_magenta().negative().rapid_blink());
    println!("{}", "--------------------------------".yellow().on_magenta().negative().rapid_blink());


    loop { // Main game loop
        term_color.reset().unwrap();
        guess_count = 0;
        let secret_animal = &animal_data[pick_random_index(animal_data.len())];
        let result_validator = Regex::new(&(r"^(?i)".to_owned() + &secret_animal.name)).unwrap();
        let mut guess_hint = Vec::new();
        let mut word_len = 0;
        for char_ in secret_animal.name.chars() {
            if char_.is_whitespace() {
                guess_hint.push(' ')
            } else {
                word_len += 1;
                guess_hint.push('-')
            }
        }
        println!("{}", "Generate a secret animal name...\n".italic());
        // println!("Pick {} as secret animal", secret_animal.name);
        // Hint
        loop {
            term_color.reset().unwrap();
            let guess_hint_str: String = guess_hint.iter().collect();
            let style_hint = format!("{} [{}]", guess_hint_str, word_len);
            if guess_count>0 {
                println!("Guess count: {} times.\n", guess_count);
            }
            println!("{}{}This animal's name has {} characters.", Colored::Fg(Color::Yellow), Attribute::Bold, word_len);
            println!("It is a {}.", secret_animal.r#type);
            println!("Its features are: {}.", secret_animal.features);
            println!("What is it?\n");
            println!("{}", "Type-in your guess:".blue().on_white().underlined());

            println!("{}{}", Colored::Fg(Color::Green), style_hint);
            let mut guess = String::new();
            read_console_input(&mut guess);
            if guess.len() != secret_animal.name.len() {
                println!("\n{}{}{}\n", Colored::Fg(Color::Red), Attribute::Bold, format!("** Input invalid. Your guess word's length must be {} characters long with spaces! **", secret_animal.name.len()));
                continue;
            }
            println!("\nYour guess is {}\n", guess);
            match check_guess(&guess, &result_validator, &input_validator, anticipate_time) {
                Guess::Right=> {
                    println!("{}{}*********Congratulation! The secret animal is the {}!*********", Colored::Fg(Color::Blue), Attribute::Bold, secret_animal.name);
                    println!("\n{}{}{}\n", Colored::Fg(Color::White), Attribute::Bold, "#".repeat(100));
                    break;
                },
                Guess::Wrong=> {
                    println!("{}Sorry, You guessed wrong!", Colored::Fg(Color::Blue));
                    let char_matched: Vec<(usize, char)> = extract_matching_char(&secret_animal.name, &guess);
                    // println!("Char Matched: {:?}", char_matched);
                    println!("There are {} characters in your guess that match the secret animal!", char_matched.len());
                    for (char_index, match_char) in char_matched {
                        guess_hint[char_index] = match_char
                    }
                    guess_count += 1; // only count guess if guess is valid.
                },
                Guess::Invalid=> {
                    println!("{}{}Input invalid. Your guess word should contain only word with space!", Colored::Fg(Color::Red), Attribute::Bold);
                }
            }
            println!("\n{}{}{}\n", Colored::Fg(Color::White), Attribute::Bold, "#".repeat(100));
        }
    }
}