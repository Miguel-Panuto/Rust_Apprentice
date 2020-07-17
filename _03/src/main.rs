use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guess the number game!");
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => break   
        }
    }
    println!("You win");
}
