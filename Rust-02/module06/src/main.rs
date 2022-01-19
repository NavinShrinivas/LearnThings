fn fn_test(_st : String){
    println!("Nothing lmao!");
}//_st looses scopt


fn main() {
    println!("Hello, world!");
    println!("The ownership model, borrowing, heap ,stack and moving");
    let mut s : String = String::from("Hello");
    s.push_str(", world!");//possible cus it's a heap allocated memory type
    //A String type : ptr to heap | len | capaciy
    let a=s;//if both a and s point to the same place on the heap
    //and as rust call the free on both a and s, this leads to a nullptr and 
    //very very unsafe.This problem is often called
    //double free error, how does rust handle this?
    //Rust simply moves s onto a, doinf this s is no longer acessible and rightfully
    //so does not need a free.This may seem like a shallow copy, but instead in Rust this
    //is called move and s is dropped.
    //println!("{}",s);//gives error
    println!("{}",a);//works
    //this behaviour DOES NOT carry over to integers and other basic types as those are stack 
    //allocated data types.Soooo, now how to deep copy? Deep copy in rust is expensive and slow 
    //as it also has to allocate space in heap
    let b = a.clone();
    println!("{}{}",a,b);
    //heres where things get harder, if you pass a String on to a function, it has fallen out of
    //scope
    fn_test(a);//a looses scope
    //println!("{}",a); //causes error

}//scope of a is cleard by calling free 
