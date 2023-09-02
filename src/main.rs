use std::io::stdin;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Failed to get input");
    println!("Your guess was {guess}");
}
