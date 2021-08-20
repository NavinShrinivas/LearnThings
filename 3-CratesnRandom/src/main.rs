use rand::prelude::*; //here we will have to make a generator
// use rand::Rng; //we simply mention the methods used by rand for increase scope

//there are defenetly more than 1 way of importing packages , all the commented code is the second
//way

fn main() {
    
    let randn : i32 = random(); //does not need a generator lol
    // let randn : i32 = rand::thread_rng().gen(); //using line 2

    let mut rng = thread_rng(); //making a generator line 1
    let randn2 : i32 = rng.gen_range(1..101); //last range is not incuded [)
    // let secret_number = rand::thread_rng().gen_range(1..101);

    println!("random numbers generates : {} {}" , randn , randn2);
}
