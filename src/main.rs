use std::{io, cmp::Ordering};

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

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal=> println!("You win!"),
    }

    println!("You guessed: {guess}");
}
