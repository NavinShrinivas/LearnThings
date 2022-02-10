use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");
    let f = File::open("./readme.md");
    let f = match f{
        Ok(file) => file,
        Err(E) => panic!("Something went wrong {}",E),
    };
    //cant use println! in the Err arm cus f is of type Result
    let f1 = File::open("newfile.txt");
    let f1 = match f1{
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => {
                println!("File not found! Creating it...");
                match File::create("newfile.txt"){
                    Ok(fc) => fc,
                    Err(er) => panic!("Panic time : {}",er),
                }
            },
            other_error => panic!("Some other unhandled errr: {:?}",other_error),
        },
    };

}
