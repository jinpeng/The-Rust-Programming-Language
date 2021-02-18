use std::io;
use rand::Rng;

fn main() {
    println!("Guessing the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);
    let mut guess = String::new();
    println!("Input your guess:");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guessed: {}", guess);
}
