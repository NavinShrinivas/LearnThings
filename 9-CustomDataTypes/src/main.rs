use std::fmt;

#[derive(Debug)]
struct Person{
    name : String,
    age : i32
}
struct Student{
    details : Person,
    dept : String,
    passoutyear : i32
}
//manual Diplay::fmt for Student struct

impl fmt::Display for Student{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{ //boiler plate code for fmt::Display
        write!(f,"Name : {} \n Age : {} \n dept : {} \n year of passing : {} \n",self.details.name,self.details.age,self.dept,self.passoutyear)
    }
}

fn main() {
    //in rust tuples are simple another type of structs , os lets first look
    //into structs
    
    //let string : String = "Navin"; this gives an error cus rust considers "" to be &str
    let string : String = String::from("Navin");//hence we have to use meothod from String :: is called a path seperator
    let age = 18;
    let navin =Person{name : string,age : age};

    println!("The defined and initilized struct of a person has name: {} and age: {}",navin.name,navin.age);

    let student1 = Student{details : navin , dept : String::from("CSE"), passoutyear : 204};
    println!("Details of a strcut whcih has a struct within it :\n {:}",student1);
    //structs cam also be broken down using the let keyword
    
    let new_person = Person{name : String::from("new guy"), age : 123};
    let Person{name : new_name , age : new_age}=new_person;
    println!("After destructureing : {} {}",new_name,new_age);
    //Note : I did not use the struct by the name of navin cus , navin was suposedly "moved"
    //in line 32 , and destructuring a moved struct is a big nono in rust
    //println!("{:?}",new_person); gives an error cus "new_person" was moved in line 37
}
