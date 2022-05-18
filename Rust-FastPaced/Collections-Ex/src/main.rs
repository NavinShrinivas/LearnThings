//Inserting to module tree
mod ModeMedian;
mod PigLatin;
mod Database;



//Imports
use std::collections::HashMap;
use std::io::*;


fn main() {
    let mut status = true;
    while status{
        println!("Crate implmenting the three exercises mentioned in the book.");
        println!("Select 1 form the below : ");
        println!("1.Find mode and median");
        println!("2.Convert to pig lating (first -> irst-fay, apple -> apple-hay)");
        println!("3.department records");
        std::io::stdout().write(b"Enter your option : ").unwrap();
        std::io::stdout().flush().unwrap();
        let mut option_input : String = String::new();
        std::io::stdin().read_line(&mut option_input).unwrap();

        match option_input.trim().parse::<i32>(){
            Ok(n)=>{
                match n{
                    1 => ModeMedian::mode_median_main::mode_median_main(),
                    2 => PigLatin::pig_latin_main::pig_latin_main(),
                    3 => Database::database_main::database_main(),
                    _ =>{ 
                        println!("\n\tWrong entry!! \n");
                        continue;
                    }                
                }
                status = false;
            },
            _ =>{
                println!("\n\tWrong entry!! \n");
            }
        };

    }
}
