use std::cmp::Ordering;
use std::io;
use std::io::Write;

use rand::Rng;

fn main() {
    println!("🛸 Welcome to Carlo's Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    let mut win: bool = false;
    let mut tries: i32 = 0;

    while !win {
        tries += 1;
        println!("--------------------------------------");
        print!("Please input your guess here: ");
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_guess) => _guess,
            Err(_) => {
                println!("Failed to read the line.");
                return ();
            }
        };

        let parsed_guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("You guessed '{}'!", guess.trim());
                num // Return.
            }
            Err(_) => {
                println!(
                    "You guessed '{}' but it's invalid. Please input a number only.",
                    guess.trim()
                );
                continue;
            }
        };

        match secret_number.cmp(&parsed_guess) {
            Ordering::Equal => {
                win = true;
                println!("\n🥳You won! After {tries} tries.");
            }
            Ordering::Greater => println!("🔼 Guess higher!"),
            Ordering::Less => println!("🔽 Guess lower!"),
        }
    }
}
