//Inserting to module tree
mod Database;
mod ModeMedian;
mod PigLatin;
//------------------------

//public modules
pub mod helper_functions {
    use std::io::*;
    pub fn flush_print(s: &str) {
        std::io::stdout().write(s.as_bytes()).unwrap();
        std::io::stdout().flush().unwrap();
    }

    pub fn read_line_self() -> String {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }
}

fn main() {
    let mut status = true;
    println!("Crate implmenting the three exercises mentioned in the book.");
    while status {
        println!("Select 1 form the below : ");
        println!("1.Find mode and median");
        println!("2.Convert to pig latin");
        println!("3.department records");
        println!("4.Exit");
        helper_functions::flush_print("Enter your option :");
        let option_input: String = helper_functions::read_line_self();

        match option_input.trim().parse::<i32>() {
            Ok(n) => {
                match n {
                    1 => ModeMedian::mode_median_main::mode_median_main(),
                    2 => PigLatin::pig_latin_main::pig_latin_main(),
                    3 => Database::database_main::database_main(),
                    4 => break,
                    _ => {
                        println!("\n\tWrong entry!! \n");
                        continue;
                    }
                }
                status = false;
            }
            _ => {
                println!("\n\tWrong entry!! \n");
            }
        };
    }
}
