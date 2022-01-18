use std::io::{self, Write};


fn main() {
    println!("Welcome to guessing game!");
    print!("Enter your guess : ");
    io::stdout().flush().expect("Failed to flush output");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed reading input");
    println!("{}",user_input);
}
