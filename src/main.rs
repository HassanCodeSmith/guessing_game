extern crate rand;

use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}", "Guess the number!".yellow());

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("{}", "Please input your guess.".purple());

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter valid integer number.".red());
                continue;
            }
        };

        println!("{}", format!("You guessed {}", guess).yellow());

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too less".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
}
