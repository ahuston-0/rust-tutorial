use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Chapter 2 of the Rust book

fn main() {
    println!("Guess the number!");

    // Generate number in range from [1,101)
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Infinite loop
    loop {
        println!("Please input your guess.");

        // Create mutable string
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // read from stdin, return Result option enum
            .expect("Failed to read line"); // If err, print this

        // Shadow string var with u32 and parse it
        // Match syntax is close to what I remember from OCaml
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Match comparison to Ordering type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too small!"),
            Ordering::Greater => println!("Your guess was too big!"),
            Ordering::Equal => {
                println!("Correct! You win!");
                break;
            }
        }
    }
}
