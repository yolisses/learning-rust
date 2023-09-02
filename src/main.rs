use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = String::from("Hellow fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
