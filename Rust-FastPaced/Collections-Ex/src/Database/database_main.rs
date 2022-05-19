use crate::helper_functions;
use std::collections::HashMap;



pub fn database_main() {
    //Basic strcture, hash map : 
    let mut databse : HashMap<String,Vec<String>> = HashMap::new();


    loop{
        helper_functions::flush_print("\tEnter command (? for help) : ");
        let command = helper_functions::read_line_self();
        let command = command.trim();
        if command == "?"{
            print_help();
        }
        else{
            let words : Vec<&str> = command.split_whitespace().collect();
            if words[0] == "Add"{
                add_records(&words,&mut databse);
            }else if words[0] == "List"{
                fetch_records(&words,&databse);
            }else if words[0] == "Exit" {
                println!("\t\tBye bye...");
                return;
            }
            else{
                println!("\t\tInvalid command, type Exit if you want to quit.")
            }
        }

    }

}

fn print_help(){
    println!("\t\tcommands available : ");
    println!("\t\t1.? : to get this very handy help section.");
    println!("\t\t2.Add <Name> to <Department> : To add <Name> to Department.");
    println!("\t\t3.List <Department> : to list all Members of a given Department." );
    println!("\t\t4.Exit : To quit the program");
}

fn add_records(words : &Vec<&str>,databse : &mut HashMap<String,Vec<String>>){
    //basic check to see if valid command was fed 
    if words.len() != 4 || words[2] != "to"{
        println!("\t\tEnter valid command, you cant get one past me.");
        return;
    }


    let value = databse.get_mut(words[3]);
    match value{
        Some(_) =>{
            value.unwrap().push(words[1].to_string());
        }
        None => {
            databse.insert(words[3].to_string(),vec![words[1].to_string()]);
        }
    }
    println!("\t\tInserted to database!");

}

fn fetch_records(words : &Vec<&str> , databse : &HashMap<String,Vec<String>>){
    //basics checks
    if words.len() !=2 {
        println!("\t\tEnter valid commands! Stop trying to break me.");
        return;
    }
    
    let value = databse.get(words[1]);
    match value{
        Some(n)=>{
            println!("\t\tAll members of {} Department : ",words[1]);
            let mut copy = n.clone(); //costly opertataion, an aviodable one that too :(. Could have used some partitioning algorithm.
            copy.sort();
            for i in copy{
                println!("\t\t{}",i);
            }
        },
        None => {
            println!("\t\t No records found for {} Department.",words[1]);
        }
    }

}
