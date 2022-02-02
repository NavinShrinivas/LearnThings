use std::fmt::format;

enum Heterogenous {
    Text(String),
    Number(i32),
    Float(f64),
}

fn main() {
    println!("Hello, world!");
    let _first_vec: Vec<i32> = Vec::new();
    let mut second_vec = vec![1, 2, 3, 4];
    second_vec.push(3); //creates a mutable refrence
    let third: i32 = second_vec[2]; //does not create a immutable borrow
    second_vec.push(3); //hence this is allowed
    let third2: &i32 = &second_vec[2]; //creates immutable borrow
                                       /*
                                        *second_vec.push(4);//not allowed
                                        */
    println!("Third element : {} {}", third, third2);
    match second_vec.get(3) {
        Some(value) => println!("4th values in the vector : {}", value),
        None => println!("Trying to access invalid index, haha you can't break me!"),
    }
    println!("Contents of the array : ");
    for (i, item) in second_vec.iter().enumerate() {
        println!(" {}th element is {}", i, item);
    }
    let mut typ: Vec<Heterogenous> = Vec::new();
    typ.push(Heterogenous::Number(6));
    typ.push(Heterogenous::Text(String::from("Hello world")));
    typ.push(Heterogenous::Float(3.4));
    println!("Contents of heterogenous vector");
    for (_i, item) in typ.iter().enumerate() {
        match item {
            Heterogenous::Number(value) => println!("Found a number : {}", value),
            Heterogenous::Text(value) => println!("Found text : {}", value),
            Heterogenous::Float(value) => println!("Floating point number : {}", value),
        }
    }
    println!("Content of array!");
    for i in &second_vec {
        println!("content of array: {}", i);
    }
    let mut new_str: String = String::from("haiii,");
    new_str.push_str("hello world");
    println!("String: {}", new_str);
    let pstr = String::from("haiiii");
    let sec_str = new_str + &pstr; //Rust is smart and converts &String to &str
                                   //here onwards new_str is moved, pstr is will valid!
                                   //println!("new_str : {}",new_str);//not allowe
    println!("sec_str : {} |||| pstr : {}", sec_str, pstr); //valid
    let str3 = format!("{}-+=[{}]+{}", pstr, pstr, pstr); //only immutable references
    println!("{} {}", str3, pstr); //allowed
}
