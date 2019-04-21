use std::io;
use std::cmp::Ordering;
use rand::Rng;

const MIN_NUM: u32 = 1;
const MAX_NUM: u32 = 101;

fn main() {
    println!("GUESS THE NUMBER GAME!");
    println!("----------------------");

    loop {
        println!("Generate secret number between {} and {}", MIN_NUM, MAX_NUM);
        let secret_number = rand::thread_rng().gen_range(MIN_NUM, MAX_NUM);

        // println!("The secret number is: {}", secret_number);
        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            match io::stdin().read_line(&mut guess) {
                Ok(string) => string,
                Err(_) => {
                    println!("Failed to read input!");
                    continue;
                }
            };

            
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You are correct!");
                    break;
                }
            }
        }
    }
}
