use std::io;
use std::io::prelude::*; 

//for now dont spend too much time processing structs
//we will get back to these soon
#[derive(Debug)]
struct nonstd(i32);

#[derive(Debug)]
struct Person <'a>{
    name: &'a str, //this is some advanced box types, lets get to this later
    age: u8
}

fn main(){
    let mut number = String::new(); //variables are immutable by default in rust
    let mut string = String::new(); //all inputs in the std::io package are always strings :(
    println!("Enter a number : ");
    io::stdin().read_line(&mut number);
    print!("Enter a sentence : ");
    
    io::stdout().flush();  //this also needs the special import in line 2
    //when using print! flush is needed nad its really weird if u ask me , read 
    // the github issue

    io::stdin().read_line(&mut string);

    //need to use parse macros to convert string to i32
    let number:i32 = number.trim().parse::<i32>().expect("Parsable");

    println!("The number inputted is : {}",number);
    println!("The sentence inputted is : {}",string);

    //below is the debug print , needed , std data types
    //can be printed using only {} or in debug mode {:?}
    //but derived data types need a implementation to be
    //written manually  , for this the fmt::Debug trait 
    //help us greatly

    //for now dont spend too much time processing structs
    //we will get back to these soon

    //debug prints on std types :
    println!("{:?} <- this was printted using a debug print","lol this looks weird");
    //if observed debug print also prints thos ""

    //non std data tpyes print , observe the "#[derive(Debug)]"
    //above the structre in line 6
    let structvar=nonstd(5);
    println!("{:?} this was a non std print",structvar);
    //try removing the derive debug in line 5 and compiler , you should get an error

    //another non std debug 
    let name = "Navin";
    let age =18;
    let structvar2=Person{name,age}; //lets look into why so later , yeah?
    println!("{:?} this is again a non std debug print",structvar2);
    println!("{:#?} this is again a non std debug print",structvar2); //pretty print
}




//from the learning guide : 

/* All types which want to use std::fmt formatting traits require an implementation to be printable.
Automatic implementations are only provided for types such as in the std library.
All others must be manually implemented somehow.

The fmt::Debug trait makes this very straightforward.
All types can derive (automatically create) the fmt::Debug implementation. 
This is not true for fmt::Display which must be manually implemented. */