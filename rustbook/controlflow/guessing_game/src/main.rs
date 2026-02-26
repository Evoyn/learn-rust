use std::io;

fn main() {
    println!("!!! Guessing Game !!!");
    println!("!!! Guess the Number !!!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Readline");

    println!("You Guessed : {guess}")
}
