//here we deal with common stl data types such as vectors(more on then) , hasmaps
//box , heaps , stacks (Possibly across 2 modules)
//
//so far all we have seen were allocated in the stack frame and buffer in the head
//specific to the function.
//this is how rust [and other languages usually] deals with variables by default ,
//i.e when the function goes out 
//scope the memory is deallocated and is cleared by os.
//
//on the other hand , heap is a memory available to the progammer and a variable stays
//in heap unless deallocated manually
//this heap memory on the other hand is accessible eveywhere by all instances , 
//as an example , objects in java are allocated in the heap and its referencing 
//data is stored in stack
//altho using heap memory is not a nice thing to do. Rust handles with this too
//
//please i am really really in the dum dum category : 
//read from here instead , its well written :
//https://www.oreilly.com/library/view/programming-rust/9781491927274/ch04.html
#[derive(Debug)]
struct points{
    x:i32,
    y:i32
}
fn main() {
    //Box types - READ lines 4-12 : its a smart pointer thats allocated in the heap
    //that derefs to values [Buffer] stored in heap memory , when the box goes out of 
    //scope both the pointer and its values are dropped 
    //
    //rust ensures a single owner , hence when the top most owner is dropped
    //all of its members are dropper irrespective of it belonging to the parent
    //or not . We will look at multiple owners soon
    let box_point : Box<points> = Box::new(points{x:5,y:4});//note that box point is pointer on heap
    let adress = &box_point;
    //let address : Box<points>=box_point;should not be done as even "partially moved values" are derefenceble in rust
    println!("address : {:p} {:p} , values : {:?}",box_point,adress,*box_point);
    //I see some abstraction in Rust in the above line , without {:p} , rust
    //automatically derefrenced the pointer hence :p must for pointer
    println!("OK now observer carefully");
    println!("Address of adress var [STACK]: {:p} \n Address of Box type [HEAP , deviation from usual]: {:p} \n Address of point type stored in box [Also in heap] : {:p}",&adress,adress,box_point);
    //observing some weird behaviour , i.e adress and &adress giving same value
    

    //collected itertaors!?!??!
    //let new_vec : Vec<i32> = (1..10); wont work , below will
    let new_vec : Vec<i32> = (1..10).collect(); //1-9
    println!("Vector : {:?}",new_vec);

    //Vectors are iterable in rust , just like pythob , but along with itertaor
    //we can also enumurate simple intergers in the same loop!
    for (i,x) in new_vec.iter().enumerate(){
        println!("In pos {} ,  we have : {}",i,x);
    }//repetition

//String ans &str
/*
 *    There are two types of strings in Rust: String and &str.
 *
 *    A String is stored as a vector of bytes (Vec<u8>), 
 *    but guaranteed to always be a valid UTF-8 sequence.
 *    String is heap allocated, growable and not null terminated.
 *
 *    &str is a slice (&[u8]) that always points to a valid UTF-8 sequence,
 *    and can be used to view into a String, just like &[T] is a view into Vec<T>.
 */
    let new_str1 : &'static str = "hello world";
    println!("String 1 : {}" , new_str1);

    for i in new_str1.split_whitespace().rev(){
        print!("{} ",i);      
    }
    println!();
    //rust can do surprinsly freaky stuff with string like : 
    //lets create a new string:
    let new_str2 : String = String::from("This is a string string hello hello");
    let mut string_vec : Vec<&str> = new_str2.split_whitespace().collect();//vector of all words in the vector
    string_vec.dedup();
    println!("{:?}",string_vec);
    

}
