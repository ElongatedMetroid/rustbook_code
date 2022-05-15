// let rust know we will use the external rand crate
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(0, 100);

    loop {
        println!("Enter a guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)   // Just like variables, references are immutable by default so we have to use mut
            .expect("Failed to read line!");    // If read_line() returns error, print this message

        // shadow the guess variable previously defined, in this case it is because we want to change the variables type
        // trim in this case is used to cut-off the newline from the number entered "5\n" -> '5'
        // parse is when used on a string type a function that converts strings into some kind of number
        let guess: u32 = match guess.trim().parse() {
            // parse returns a result type, usually if it hits Err the user entered a letter into the stdin
            // success on turning the string into a number; return the number to the guess variable and continue normaly
            Ok(num) => num,
            // failure on turing the string into a number; skip the rest of the loop and go back to the top of it
            Err(_) => continue,
        };
            

        println!("You guessed {}", guess);

        // compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                // break out of the loop
                break;
            },
        }
    }
}
