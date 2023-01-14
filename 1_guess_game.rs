use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(1..101);
    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        println!("Input your answer: ");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
        println!("Input: {}", guess);
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid number!"); continue;}
        };
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed correctly! It is {}!", secret);
                break;
            }
        }
    }
}
