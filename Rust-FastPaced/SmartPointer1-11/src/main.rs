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

use std::rc::Rc;
#[derive(Debug)]
enum BetterList{
    Cons(i32,Rc<BetterList>),
    Nil
}

use std::cell::RefCell;
#[derive(Debug)]
enum BetterBetterList{
    Cons(Rc<RefCell<i32>>,Rc<BetterBetterList>),
    Nil
}

use crate::List::{Cons,Nil};
use crate::BetterList::{Cons as BLCons, Nil as BLNil};
use crate::BetterBetterList::{Cons as BLBLCons, Nil as BLBLNil};
//BL standing for betterList cus we have two Cons and 2 Nil types in this file 

use std::ops::Deref;
#[derive(Debug)]
struct OurBox<T : std::fmt::Debug> (T); //A tuple struct

struct OurBox2<T> (T); //A tuple struct

impl<T : std::fmt::Debug> OurBox<T>{
    fn new(x : T) -> Self{
        OurBox(x)
    }
}

impl<T> OurBox2<T>{
    fn new(x : T) -> Self{
        OurBox2(x)
    }
}


impl<T : std::fmt::Debug> Deref for OurBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl<T : std::fmt::Debug> Drop for OurBox<T>{
    fn drop(&mut self) {
        println!("Printed from drop trait as variable is out of scope : {:?}",self.0);
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
    //Do note, Nil here is simply a enum type and has nothing to do with NULL or undefined or any
    //such concepts
    println!("{:#?}",psuedo_linked_list);


    //refrences and Deref trait
    let x = 5;
    let y = &x;
    println!("{}",y); //Refrence coersion, the same coersion doesnt work with assert eq
    let new_box_type = OurBox::new(String::from("Hello world!"));
    let new_box_ref = &new_box_type;
    //drop(new_box_type); //Wont work as new_box_type is considered moved here, hence all the rules of moving remains true
    println!("{:?}",*new_box_ref);
    let new_box_type2 = OurBox2::new(String::from("Hello world!"));
    assert_eq!(*new_box_type,String::from("Hello world!"));//Without deref the * wont work
    //assert_eq!(*new_box_type2,String::from("Hello world!"));//wont work



    //Deref coersion

    //ref_str_printer(&new_box_type2); //Wont work 
    ref_str_printer(&new_box_type);//Works


    //Refrence counting
    let list_a : List = Cons(5,Box::new(Cons(10,Box::new(Nil))));
    let list_b : List = Cons(3,Box::new( list_a )); //Transfers ownership of a 
    //println!("{:?}",list_a); //Hence, this wont work
    //let list_c : List = Cons(4,Box::new( list_a )); //And neither will this
    


    let list_a : Rc<BetterList> = Rc::new(BLCons(5,Rc::new(BLCons(10,Rc::new(BLNil)))));
    move_value_check(Rc::clone(&list_a));
    let list_b : BetterList = BLCons(3,Rc::clone( &list_a )); 
    println!("from main {:?}",list_a); 
    let list_c : BetterList = BLCons(4,Rc::clone( &list_a )); 
    println!("Full list : {:?}",list_c);

    //seeing refrence count in action
    let a : Rc<BetterList> = Rc::new(BLCons(5,Rc::new(BLCons(10,Rc::new(BLNil)))));
    println!("Just after creating a : {}",Rc::strong_count(&a));
    move_value_check(Rc::clone(&list_a));
    println!("Just after creating trying to move value : {}",Rc::strong_count(&a));
    let b : BetterList = BLCons(3,Rc::clone( &a ));
    println!("Just after creating b : {}",Rc::strong_count(&a));
    let c : BetterList = BLCons(4,Rc::clone( &a )); 
    println!("Just after creating c : {}",Rc::strong_count(&a));
    println!("Full list : {:?}",list_c);


    //RefCell with Rc
    println!("multiple Mutable refrences use rc and recell \n");
    let value = Rc::new(RefCell::new(10));
    let a : Rc<BetterBetterList> = Rc::new(BLBLCons(Rc::new(RefCell::new(10)),Rc::new(BLBLCons(Rc::clone(&value),Rc::new(BLBLNil)))));
    println!("Just after creating a : {}",Rc::strong_count(&a));
    let b : BetterBetterList = BLBLCons(Rc::new(RefCell::new(3)),Rc::clone( &a ));
    println!("Just after creating b : {:?}",b);
    let c : BetterBetterList = BLBLCons(Rc::new(RefCell::new(4)),Rc::clone( &a )); 
    println!("Just after creating c : {:?}",c);

    println!("After modifying usinf 2 mutable refrences : \n");

    *value.borrow_mut()=20;
    *value.borrow_mut()=50;
    println!("list : {:?}",c);
    //So now we can have multiple mutable refrences and multiple imut refrences (like ownership)!!


}

fn move_value_check(waste : Rc<BetterList>){
    println!(" from value mover {:?}", waste);
    //even tho a reference was sent wrapped in Rc::clone, this function will NEVER KNOW THAT!!!
    return
}

fn ref_str_printer(input : &str){
    println!("Hello, {}",input);
}
