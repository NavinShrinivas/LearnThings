use std::thread::sleep;
use core::time::Duration;


struct Cacher<T>{
    calculation : T,
    value : Option<u32>
}

impl<T> Cacher<T>
where
    T : Fn(u32) -> u32,
    //Note Fn is a trait fn is the usual keyword
{
    
}

fn main() {
    println!("Hello, world!");
    //Closures first
    let var = String::from("Hello, World!");
    let first_closure = |var : String| {
        sleep(Duration::from_secs(2));
        println!("{}",var);
    };

    let second_closure = |num|{
        //something here
        println!("{}",num);
    };
    second_closure(String::from("Hello"));
    //second_closure(3); //Doesn't work
}
