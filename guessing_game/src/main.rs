use rand::Rng;
use std::cmp::Ordering;
use std::io;

// main
fn main() {
    println!("Guess the number!"); // Macro

    let secret_number = rand::rng().random_range(1..=100); // Immutable

    // Spoilor!
    // println!("The secret number is: {secret_number}");

    // while(1)
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // String

        // User input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // catch(...) {}

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
