use std::collections::HashMap;
use std::io::*;

pub fn mode_median_main(){
    let mut input_numbers : Vec<i32> = Vec::new();
    let mut input_numbers_hash : HashMap<i32,i32> = HashMap::new();
    println!(""); //Formatting


    flush_print("\tInput all element in one line seperated by space : ");
    let raw_number = read_line_self();
    let input_numbers_str : Vec<&str> = raw_number.split_whitespace().collect();
    let mut n = 0;
    for i in input_numbers_str{
        n+=1;
        match i.parse::<i32>(){
            Ok(e)=>{
                let count = input_numbers_hash.entry(e).or_insert(0);
                *count+=1;
                input_numbers.push(e);
            },
            _ => {
                println!("Ahh ah! I found a wrong entry, you thought you could fool me?");
            }
        }
    }
    let mut mode : i32 = 0;
    for i in &input_numbers_hash{
        let current_max : i32 = match input_numbers_hash.get(&mode){
            Some(n) => *n,
            _=>0
        };
        if i.1 > &current_max{
            mode = *i.0;
        }
    }
    println!("\tMode : {}",mode);
    input_numbers.sort();
    if n%2 == 0{
        println!("\tMedian : {}",(f64::from(input_numbers[n/2])+ f64::from(input_numbers[(n/2)-1]))/2.0)
    }else{
        println!("\tMedian : {}",input_numbers[n/2]);
    }
}

fn flush_print(s : &str){
    std::io::stdout().write(s.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();
}

fn read_line_self()->String{
    let mut input : String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
