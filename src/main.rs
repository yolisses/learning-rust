use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guess the number!");
    let random_number = rand::thread_rng().gen_range(0..=100);
    println!("The random number is {random_number}");

    println!("Please input your guess.");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Failed to get input");
    let guess: u32 = guess.trim().parse().expect("Failed to convert to number");
    println!("Your guess was {guess}");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You win!"),
    }
}
