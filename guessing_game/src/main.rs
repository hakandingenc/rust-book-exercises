extern crate rand;

use std::io;
use std::cmp::Ordering; // An enum type
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // Rust defaults to i32 for number type
    // thread_rng gives random number generator
    // gen_range generates a number in the specified range using the RNG

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // String : growable, UTF-8 encoded text
        // new : associated function of the String type (static method)
        io::stdin()
        .read_line(&mut guess) // Needs to be mutable
        .expect("Failed to read line"); // read_line returns `Result`

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is called `catchall value`
        };
        // Note the semicolon
        // Type must be given because context is needed for inference
        // Rust has a strong, static type system.

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // Pattern matching for enum type
            // Not covering all cases is an error
            Ordering::Less => println!("Too small!"), // Each line is called `an arm`
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } // Trailing commas are allowed
        }
    }
}
