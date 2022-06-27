/*
 *Cons list is from lisp language and is a recursive data structure
 *The concept of Cons is explained in the rust book
 */
#[derive(Debug)]
enum List{
    /*
     *Cons(i32,List),//Wont work, this is cus rust cant figure out how much space to allocate for List
     *in the stack, hence its time to start using the heap.
     */
    Cons(i32,Box<List>), //This works!
    Nil
}
use crate::List::{Cons,Nil};


use std::ops::Deref;
struct OurBox<T> (T); //A tuple struct

struct OurBox2<T> (T); //A tuple struct

impl<T> OurBox<T>{
    fn new(x : T) -> Self{
        OurBox(x)
    }
}

impl<T> OurBox2<T>{
    fn new(x : T) -> Self{
        OurBox2(x)
    }
}


impl<T> Deref for OurBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn main() {
    let first_box : Box<u32> = Box::new(9834587);
    println!("{}",first_box); //meaning Box types have fmt print impl
                            

    let new_value = 123; //stack allocated, doesnt matter even if it was heap allocated 
    let box_new_value = Box::new(new_value);//does now Move new_value 
    println!("{}",new_value); //valid
    let another_new_value = String::from("test string");
    let another_new_box = Box::new(another_new_value);
    //println!("{}",another_new_value); //Invalid

    //recursive types in boxes 
    let psuedo_linked_list : List = Cons(12,Box::new(Cons(23,Box::new(Cons(34,Box::new(Nil))))));
    println!("{:#?}",psuedo_linked_list);


    //refrences
    let x = 5;
    let y = &x;
    println!("{}",y); //Refrence coersion, the same coersion doesnt work with assert eq
    let new_box_type = OurBox::new(String::from("Hello world!"));
    let new_box_type2 = OurBox2::new(String::from("Hello world!"));
    assert_eq!(*new_box_type,String::from("Hello world!"));//Without deref the * wont work
    //assert_eq!(*new_box_type2,String::from("Hello world!"));//wont work
}