

fn main() {
    //if you remember , i mentioned all varibles are immutable by default in rust 
    //but u can change value of even immutable varibles using a method called "shadowing"
    //like : 
    let a : i32 =10; //immutable
    let a=a*    2; //works
    let a=a+10; //work 
    //the above two lines worked only cus of the presence of `let` keyword
    println!("Value of a : {}",a);

    //lets look into constants , these are thos that are always immutable , cant be overshadowed 
    //and always have to have types annotated to them
    const b : &str = "hello world";

    let v : i32 = "234".parse().expect("Hmmmm , Not a number ?"); //observe i havent done <i32> as rust
    //is insanely smart and can figure out my output type :)
    println!("Value of parsed string to int : {}",v);


}

/* integer overflow : 

Let’s say you have a variable of type u8 that can hold values between 0 and 255. 
If you try to change the variable to a value outside of that range, such as 256, 
integer overflow will occur. Rust has some interesting rules involving this behavior. 
When you’re compiling in debug mode, 
Rust includes checks for integer overflow that cause your program to panic at runtime if 
this behavior occurs. Rust uses the term panicking when a program exits with an error; 
we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.
When you’re compiling in release mode with the --release flag, 
Rust does not include checks for integer overflow that cause panics.
Instead, if overflow occurs, Rust performs two’s complement wrapping. 
In short, values greater than the maximum value the type can hold “wrap around” to the minimum 
of the values the type can hold. In the case of a u8, 256 becomes 0, 257 becomes 1, and so on. 
The program won’t panic, but the variable will have a value that probably isn’t what you were 
expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error. */

