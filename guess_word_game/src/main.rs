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
    Right(String),
    Wrong(String),
    Invalid(String)
}

enum GameCommand {
    PlayerInput(String),
    Next(String),
    Quit(String),
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

fn check_game_command(gameinput: &str) -> GameCommand {
    if gameinput.eq_ignore_ascii_case("next") {
        return GameCommand::Next("Session Skip!".to_string())
    }
    if gameinput.eq_ignore_ascii_case("quit") {
        return GameCommand::Quit("Game Exit!".to_string())
    }
    GameCommand::PlayerInput(gameinput.to_string())
}

fn check_guess(guess: &str, anwser: &str, thinking_duration: time::Duration) -> Guess {
    let result_validator = Regex::new(&(r"^(?i)".to_owned() + &anwser)).unwrap();
    let input_validator = Regex::new(r"^([A-Za-z-]+\s*)+$").unwrap();
    thread::sleep(thinking_duration);
    if guess.len() != anwser.len() {
        return Guess::Invalid(format!("Input invalid. Your guess word's length must be {} characters long with spaces!", anwser.len()).to_string())
    }
    println!(".");
    thread::sleep(thinking_duration);
    if !input_validator.is_match(&guess) {
        return Guess::Invalid("Input invalid. Your guess word should contain only word with space!".to_string())
    }
    println!("..");
    thread::sleep(thinking_duration);
    if !result_validator.is_match(&guess) {
        return Guess::Wrong("You guessed wrong!".to_string())
    }
    println!("...");
    thread::sleep(thinking_duration);
    Guess::Right("Congratulation! The secret animal is the {}!".to_string())
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

fn generate_animal_data() -> Vec<Animal> {
    let animal_data_string = include_str!("animal_datas.json");
    let animal_data: Vec<Animal> = serde_json::from_str(&animal_data_string).unwrap_or_else(|error| {
        panic!("Something wrong when parsing data {:?}", error)
    });
    animal_data
}


fn gameloop() {
    // Initialize Terminal
    let cterm = Crossterm::new();
    let terminal = cterm.terminal();
    let term_color = cterm.color();
    terminal.clear(ClearType::All).unwrap();
    env_logger::init();

    // Initialize Game Variable
    let animal_data = generate_animal_data();
    let mut score: i64 = 10;
    let MaxGuessCount = 12;
    let mut guess_count: i64;
    let anticipate_time = time::Duration::from_millis(500);
    debug!("Animal Database: {:?}", animal_data);
    debug!("Result anticipate duration: {:?}", anticipate_time);
    // Main game loop
    loop {
        term_color.reset().unwrap();
        if score <= 0 {
            println!("Game Over!");
            break;
        }
        guess_count = 0;
        let secret_animal = &animal_data[pick_random_index(animal_data.len())];
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
        // Session game loop
        loop {
            term_color.reset().unwrap();
            let guess_hint_str: String = guess_hint.iter().collect();
            let style_hint = format!("{} [{}]", guess_hint_str, word_len);
            println!("Score: {}.\n", score);
            if guess_count>0 {
                println!("Guess count: {} times.\n", guess_count);
            }
            println!("{}{}This animal's name has {} characters.", Colored::Fg(Color::Yellow), Attribute::Bold, word_len);
            println!("It is a {}.", secret_animal.r#type);
            println!("Its features are: {}.", secret_animal.features);
            println!("What is it?\n");
            println!("{}", "Type-in your guess:".blue().on_white().underlined());

            println!("{}{}", Colored::Fg(Color::Green), style_hint);
            let mut playerinput = String::new();
            read_console_input(&mut playerinput);
            match check_game_command(&playerinput) {
                GameCommand::Next(next_msg)=> {
                    println!("{}", next_msg);
                    score -= 1;
                    break;
                }
                GameCommand::Quit(quit_msg)=> {
                    println!("{}", quit_msg);
                    std::process::exit(0);
                }
                GameCommand::PlayerInput(player_input)=> {
                    println!("\nYour guess is {}\n", player_input);
                }
            }
            match check_guess(&playerinput, &secret_animal.name, anticipate_time) {
                Guess::Right(right_msg)=> {
                    println!("{}{}*********{}*********", Colored::Fg(Color::Blue), Attribute::Bold, right_msg);
                    println!("\n{}{}{}\n", Colored::Fg(Color::White), Attribute::Bold, "#".repeat(100));
                    score += MaxGuessCount-guess_count;
                    break;
                },
                Guess::Wrong(wrong_msg)=> {
                    println!("{}{}", Colored::Fg(Color::Blue), wrong_msg);
                    let char_matched: Vec<(usize, char)> = extract_matching_char(&secret_animal.name, &playerinput);
                    // println!("Char Matched: {:?}", char_matched);

                    println!("There are {} characters in your guess that match the secret animal!", char_matched.len());
                    for (char_index, match_char) in char_matched {
                        guess_hint[char_index] = match_char
                    }
                    guess_count += 1; // only count guess if guess is valid.
                },
                Guess::Invalid(invalid_msg)=> {
                    println!("{}{}{}", Colored::Fg(Color::Red), Attribute::Bold, invalid_msg);
                }
            }
            println!("\n{}{}{}\n", Colored::Fg(Color::White), Attribute::Bold, "#".repeat(100));
        }
    }
}

// Main 
fn main() {

    // Render Game Title
    println!("{}", "--------------------------------".yellow().on_magenta().negative().rapid_blink());
    println!("{}", "-----Guess the Animal Game------".yellow().on_magenta().negative().rapid_blink());
    println!("{}", "--------------------------------".yellow().on_magenta().negative().rapid_blink());

    gameloop();
}