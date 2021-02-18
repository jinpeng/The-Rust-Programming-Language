use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret number is {}", secret_number);
    loop {
        println!("Input your guess:");
        let mut guess_str = String::new();
        match io::stdin().read_line(&mut guess_str) {
            Ok(x) => x,
            Err(_) => {
                println!("Failed to read line from standard input.");
                continue;
            }
        };
        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type an integral number.");
                continue;
            }
        };
        println!("Your guessed: {}", guess);
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
