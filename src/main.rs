use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Guess the number!");
    let number = rand::thread_rng().gen_range(0..=100);
    println!("The random number is {number}");

    println!("Please input your guess.");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Failed to get input");
    println!("Your guess was {guess}");
}
