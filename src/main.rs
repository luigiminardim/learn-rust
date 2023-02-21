use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 * This is a simple guessing game.
 * The program will generate a random number between 1 and 100.
 * The user will then be prompted to guess the number.
 * The program will then tell the user if their guess is too high or too low.
 * If the user guesses the correct number, the program will print a congratulatory message.
 */
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess.");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        let result = guess.cmp(&secret_number);
        match result {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
