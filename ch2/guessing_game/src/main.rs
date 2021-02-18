use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);
    loop {
        println!("Input your guess:");
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");
        println!("Your guessed: {}", &guess_str);
        let guess :u32 = guess_str
            .trim()
            .parse()
            .expect("Please type an integral number!");
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
