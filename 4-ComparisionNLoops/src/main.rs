//here we will be implementing a guessing game using almost everything we have used so far
use std::io;
use rand::prelude::*;
use std::io::prelude::*; 
use std::cmp::Ordering;

fn main() {
    //time to create our randnumber generator
    let mut rng=thread_rng();
    let value=rng.gen_range(1..101);

    loop{
        print!("Enter your guess : ");
        io::stdout().flush();
        let mut users_guess=String::new();
        io::stdin().read_line(&mut users_guess);
        let users_guess_int : i32 = users_guess.trim().parse::<i32>().expect("Pleae enter a parsable value , i.e number");

        match users_guess_int.cmp(&value){
            Ordering::Less => println!("Your guess is small , think big!"),
            Ordering::Greater => println!("Your guess is to big."),
            Ordering::Equal => {println!("your got it!");break;}
        }

    }
    //if / else constructs\

    let choice : i32 = 234;
    if choice % 2!=0 {
        println!("{} is a odd number ",choice);
    }
    else{
        println!("{} is a even number ",choice);
    }

}
