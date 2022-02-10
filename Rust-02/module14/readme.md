# Module 14 : Recoverable errors and more about panic! 

This module remains as a gateway to the most hard module in the rust chapter which begins in module15, enjoy this module when you can :) 

## Recoverable Errors using Result<T,E>

The Result enum : 
```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
say when opening a file, the file does not exists, this is a recoverable error, Results can handle this, now the file open function must return Result right? How do we know it does? Either you can check the official API documentation or depend on the errors from Rust compiler, why did I type this long ass paragraph?? It's show that compiler errors are part of Rust development cycle unlike other languages.

To be more accurate, the File (method?) thing returns : Result<File, std::io::Error>

### Error matching 

Err types in rust have a .kind() method that can be passed in a match construct and fancy things be achieved.
In this case we used a lotttt of match statement kinda making the code messy and unreadable, luckily Result enum comes with multiple impl for simplifying this process, which we will be looking to later. An experienced Rustacean might do this : 
```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```
Neat no? For now let's look into unwrap and except 

### Unwrap and Except 

Unwrap will return the value if it's of type Ok, else it calls panic. Except is also similar to Unwrap, but we can specify what message it will be raising the panic using
