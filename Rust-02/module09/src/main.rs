use std::{
    io::{self, prelude::*, Write},
    process::exit,
};

struct firststruct {
    name: String,
    age: i32,
    education: String,
    alive: bool,
}

impl firststruct {
    fn alivecheck(&self) {
        if self.age <= 80 {
            println!("{} is alive?{}", self.name, true)
        }
    }
}
//tuple struct  :
struct firsttstruct(String, u32, bool);

#[derive(Debug)]
struct rect_data {
    length: i32,
    breadth: i32,
    area: i32,
}

impl rect_data {
    fn area_method(&self) {
        println!("Area : {}", self.length * self.breadth);
    }
}

fn build_usr(name: String, age: i32, education: String) -> firststruct {
    firststruct {
        name,
        age,
        education,
        alive: if age <= 80 { true } else { false },
    }
    //here we are not use the key : value notions, how and why? well this is called init shorthand
    //notation and is implmented in rust to make building structs easier.
    //it works only if the value (parameter variable) is the same as the key(property) value.
}

fn build_rec() -> rect_data {
    print!("Enter length : ");
    io::stdout().flush();
    let mut length = String::new();
    io::stdin().read_line(&mut length);
    let length: i32 = match length.trim().parse() {
        Ok(e) => e,
        Err(_) => {
            println!("Please stop trying to break me");
            exit(0)
        }
    };
    print!("Enter breadth : ");
    io::stdout().flush();
    let mut breadth = String::new();
    io::stdin().read_line(&mut breadth);
    let breadth: i32 = match breadth.trim().parse() {
        Ok(e) => e,
        Err(_) => {
            println!("Please stop trying to break me");
            exit(0)
        }
    };

    rect_data {
        length,
        breadth,
        area: 0,
    }
}

fn calculate_area(r: &rect_data) -> i32 {
    r.length * r.breadth
}

fn main() {
    println!("Hello, world!");
    let mut first_person = firststruct {
        name: String::from("Newton"),
        age: 145,
        education: String::from("PhD"),
        alive: false,
    };
    first_person.age = 123;
    //for this the entire struct must be mut, specific properties of structs cant be mut only.
    println!("Age of {} is {}", first_person.name, first_person.age);
    let mut secondperson = build_usr(String::from("Navin"), 19, String::from("btech"));
    secondperson.age = 18;
    println!("{} is alive? {}", secondperson.name, secondperson.alive);

    //without update syntax
    let person3 = firststruct {
        name: first_person.name,
        age: 12,
        education: first_person.education,
        alive: first_person.alive,
    };
    //suprsingly, this also moves the values from the first struct to the newer one and older
    //this nmakes sense, cus the Strings within the struct are heap stored memory and need
    //proper free calling to avoid double free erro.
    //this is rightfully and exactly how its mentioned and explaing in the book
    //varaibles can no longer be accessed, so ill be using another strcut to show update syntax
    //with update syntax :
    let age = 12;
    let person4 = firststruct {
        age,
        ..secondperson
    };
    let tstruct = firsttstruct(String::from("Hello"), 12, false);
    let mut rect1: rect_data = build_rec();
    rect1.area = calculate_area(&rect1);
    println!("Area of rectangle : {}", rect1.area);
    println!("{:#?}", rect1);
    person4.alivecheck();
    rect1.area_method();
}
