use std::io;

use rand::Rng;

fn main() {

    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    println!("Secret number: {secret_number}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
