use rand::Rng;
use std::{
    io::{self, prelude::*},
    process::exit,
};

fn main() {
    let mut rng = rand::thread_rng();
    let randnum = rng.gen_range(0..100);
    println!("Welcome to guessing game!");
    println!("Hmm I have a number in mind b/w 0 and 100, try and guess it in under 10 guesses.");
    let mut guesses : i32 = 10;
    while guesses > 0 {
        println!("Number of guesses left : {}",guesses);
        print!("Enter your guess : ");
        io::stdout()
            .flush()
            .expect("Something went wrong when flushing output.");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong with reading input");
        println!("Guessed number : {}", guess);
        /*
         *match guess.cmp(&randnum){
         *    std::cmp::Ordering::Less => println!("Underguessing."),
         *    std::cmp::Ordering::Equal => {
         *        println!("You got it!");
         *        exit(0)
         *    },
         *    std::cmp::Ordering::Greater => println!("Overguessing"),
         *    _=>println!("Something went wrong on our side")
         *}
         */
        guesses-=1;
        let guess: i32 = match guess.trim().parse() { //previous is being shadowed
            Ok(n) => n,
            Err(_) => continue, //i.e continues with the looping
        };
        match guess.cmp(&randnum) {
            std::cmp::Ordering::Less => println!("You are underguessing."),
            std::cmp::Ordering::Equal => {
                println!("Damn,You got it!");
                break;
            }
            std::cmp::Ordering::Greater => println!("You are overguessing"),
            _ => println!("Something went wrong on out side."),
        }
    }
}
