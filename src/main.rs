use rand::Rng;
use std::{cmp::Ordering, io}; //puts this std input/ouput library for Rust into this file

fn main() {
    // In Rust, fn main() is the entry point of a Rust program. It is the starting point of execution when the program is run. The main function is where the program's execution begins and it is mandatory in every Rust executable.
    println!("Guess the number!"); // Print the string "Guess the number" to the console

    let secret_number = rand::thread_rng().gen_range(1..=100); // generates a secret number between 1 and 100 using the rand library

    println!("The secret number is: {secret_number}"); // prints the secret number for development

    loop {
        println!("Please input your guess."); // Print the string "Please input your guess." to the console

        let mut guess = String::new(); // Declare a mutable variable `guess` of type `String` and initialize it with a new empty string using ::new, this also creates storage for the user input

        io::stdin() // Access the standard input stream
            .read_line(&mut guess) // Read a line from the standard input and store it in the `guess` variable, modifying its value
            .expect("Failed to read line"); // If reading the line fails, print the error message "Failed to read line"

        let guess: u32 = match guess.trim().parse() {
            // Parse the input string `guess` into an unsigned 32-bit integer (`u32`)
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); // Print the string "You guessed: " followed by the value of `guess` to the console

        match guess.cmp(&secret_number) {
            // Compare `guess` with `secret_number` using the `cmp` method
            Ordering::Less => println!("Too small!"), // If `guess` is less than `secret_number`, print "Too small!"
            Ordering::Greater => println!("Too big!"), // If `guess` is greater than `secret_number`, print "Too big!"
            Ordering::Equal => {
                println!("You win!"); // If `guess` is equal to `secret_number`, print "You win!"
                break; // ends the operation and in this case the program
            }
        }
    }
}

// Remembedr to run the command cargo doc --open to get easy documentation of the libraries and functions you are using.
