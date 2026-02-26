use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!!! Guess the Number !!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid type a number please");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Kekecilan, Kamu Kalah"),
            Ordering::Equal => {
                println!("Wow you win Secret number is {secret_number}");
                break;
            }
            Ordering::Greater => println!("Kebesaran, Kamu Kalah"),
        }
    }
}
