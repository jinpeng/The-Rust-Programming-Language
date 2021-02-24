use std::io;

fn convert_to_pig_latin(txt: &String) -> String {
    let first_char = txt.chars().next().unwrap();

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => txt.to_string() + "-hay",
        _ => txt[1..].to_string() + &first_char.to_string() + "ay",
    }
}

fn main() {
    let mut input = String::new();
    loop {
        println!("Input a string:");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                break;
            },
            Err(_) => {
                println!("Failed to read line frome standard input");
                continue;
            }
        };
    }
    input = input.trim().to_string();
    let pig_latin = convert_to_pig_latin(&input);
    println!("Origin: {}, Pig latin: {}", input, pig_latin);
}
