use std::io; //puts this std library for Rust into this file

fn main() {
    println!("Guess the number");

    println!("Please input your game.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}