use std::fmt;
// use std::io;
// use std::io::prelude::*;

#[derive(Debug)]
struct Points(i32,i32);

impl fmt::Display for Points{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ //boiler plate code for fmt::Display
        write!(f,"({},{})",self.0,self.1) //returning this line to fmt::Result , will look into this
        //under a module in book , I am combining book as well examples as the learning curve in
        //book in really flat :(
    }
}

fn main() {
    // println!("Hello, world!");

    let coordinates = Points(2,-5);
    println!("Debug : {:?}",coordinates); //wont work without line 5
    println!("Display : {}",coordinates);
}
