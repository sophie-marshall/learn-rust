
use std::io; 
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    
    println!("Guess the number!");

    // generate number between 1 and 100, inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // start guess loop
    loop {
        
        println!("Please input your guess.");
    
        // insantiate guess variable (immutable by default)
        let mut guess = String::new();
    
        // use standard input to collect user guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // handle case of bad input
    
        // coerce into integer for comparison
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        // compare guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => {
                // success message + loop break
                println!("You win!");
                break;
            }
        }
    }

}
