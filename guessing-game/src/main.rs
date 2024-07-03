use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut input_string = String::new();

        println!("Guess a number!");

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        println!("You guessed: {}", input_string);

        let numeric_input: u32 = match input_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match numeric_input.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".yellow()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
