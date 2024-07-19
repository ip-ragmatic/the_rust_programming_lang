use std::{io, cmp::Ordering};
use rand::{thread_rng, Rng};

fn main() {
    let secret_num = thread_rng().gen_range(1..=100);

    println!("Guess a number between 1-100:");
    println!("The secret number is ... {secret_num}");

    loop {
        println!("Enter a number.");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("I'm unable to read your input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}