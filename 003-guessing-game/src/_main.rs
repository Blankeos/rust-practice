/**
 * This is the file that I used when I didn't know how to write Rust. it completely
 * follows the tutorial + some notes I learned about the language here.
 *
 * `main.rs` is the file that I wrote knowing Rust without the tutorial and just my mind alone.
 */
// 📝 Lesson: Import ::{self, Write}; is added so we can use flush().
// But actually, this is a 1-line shortcut for:
//  - std::io (as usual).
//  - std::io::Write.
// Because std::io::{self} means import `io`. Then `, Write}` after that is just std::io::Write.
// Because if you do `std::io::Write` only, you don't import `io` lol.
use rand::Rng; // Rng trait defines methods that random number generators implement. Must be in scope for us to use gen_range.
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("I have generated a random number. Try to guess it!");

    // 1..=100 is "Inclusive Range". While 1..100 is "Exclusive". (Refers to if the last element will be included in the range).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    let mut win: bool = false;

    while !win {
        println!("--------------------------------------");
        let mut guess = String::new(); // 📝 Lesson: `::new()` is an "associated function" - implemented on the type.

        print!("Please input your guess: ");
        // Makes sure to display `print!` before io:stdin
        // 📝 Low-level lesson:
        // - With stdout, outputs are actually sent and stored to the buffer first,
        // then flushed (sent to the terminal) either when "buffer is full" or "\n is encountered".
        // - When you `print!`, there is no '\n' at the end. So it just sits in the buffer.
        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // 📝 'Match' is like an if else with a return that you can use in an assignment.
        let parsed_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to type a number. Found: {guess}");
                continue;
            }
        };

        println!("You have guessed {parsed_guess}");

        // 📝 Match is used like a switch case here based on what `cmp` returns.
        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Equal => {
                win = true;
                println!("You win!")
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
