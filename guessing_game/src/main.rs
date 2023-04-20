use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("type err or ...:{e}");
                continue;
            }
        };
        // Rust allows us to shadow the previous value of guess with a new one.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("TOO BIG"),
            Ordering::Equal => {
                println!("YOU WIN");
                // quit loop
                break;
            }
        }
    }
}
