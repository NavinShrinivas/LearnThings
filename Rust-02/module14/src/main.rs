use std::fs::File;
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::io::{self, prelude::*};


fn stringintparse()-> Result<i32,ParseIntError>{
    print!("Enter a number : ");
    io::stdout().flush().expect("Something went wrong when printing");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Something went wrng when reading");

    match s.trim().parse(){
        Ok(value) => Ok(value),
        Err(e) => Err(e)
    }

}

fn simplerstringintparse()->Result<i32,ParseIntError>{
    print!("Enter a number : ");
    io::stdout().flush().expect("Something went wrong when printing");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Something went wrng when reading");
    let res : i32 = s.trim().parse()?;
    Ok(res)
    
}

fn main() {
    println!("Hello, world!");
    let f = File::open("./readme.md");
    let f = match f {
        Ok(file) => file,
        Err(E) => panic!("Something went wrong {}", E),
    };
    //cant use println! in the Err arm cus f is of type Result
    let f1 = File::open("newfile.txt");
    let f1 = match f1 {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("File not found! Creating it...");
                match File::create("newfile.txt") {
                    Ok(fc) => fc,
                    Err(er) => panic!("Panic time : {}", er),
                }
            }
            other_error => panic!("Some other unhandled errr: {:?}", other_error),
        },
    };
    let f2: File = File::open("newfile.txt").unwrap(); //should return back file descriptor directly
                                                       //unwarp makes it harder to dedbug with no panic message, hence except
    /*
     *let f3: File =
     *    File::open("reallynewfile.txt").expect("Ahh, something went wrong when opening the file");
     */
    match stringintparse(){
        Ok(value) => println!("Value entered as an i32 : {}",value),
        Err(e) => println!("Propageted error : {}",e)
    }
    match simplerstringintparse(){
        Ok(value) => println!("Value entered as an i32 : {}",value),
        Err(e) => println!("Propageted error : {}",e)
    }
}
