
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

fn main() {
    let first_box : Box<u32> = Box::new(9834587);
    println!("{}",first_box); //meaning Box types have fmt print impl
                            
    //recursive types in boxes 
    let psuedo_linked_list : List = Cons(12,Box::new(Cons(23,Box::new(Cons(34,Box::new(Nil))))));
    println!("{:#?}",psuedo_linked_list);
}
