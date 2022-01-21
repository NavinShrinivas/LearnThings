use std::io;
use std::io::prelude::*;

fn oldwordfn(s : &String)->usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item==b' '
        {
            return i; 
        }
    }
    return s.len();
}


fn newwordfn(s : &String)->&str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item==b' '
        {
            return &s[0..i]; 
        }
    }
    &s[..]

}

fn somefn(s: &String)->&str{
    &s[..3]
}


fn main() {
    println!("Hello, world!");
    let mut s : String = String::from("hmmmm");
    //let space = oldwordfn(&s);
    //say we did s.clear herem space would still have a value that makes sense to previous value of
    //string, this is tedious to track , hence slices.
    //slices are redable refrence only
    print!("Enter a senctence : ");
    io::stdout().flush().expect("Something went wrong");
    let re = somefn(&s);
    io::stdin().read_line(&mut s).expect("Something went wrong");
    //s.clear();//no error
    let slice = newwordfn(&s);//does not give erro , even thos there already exists a mutablle borroe, this is cus
    //println!("{}",re); //gives error

    //FINALLY : stuff is made clear, you can not use a immutable refrence that was created before a
    //mutable borrw or refrence was made.




    //rust can tell that the mutable borrow is not used anymore and its safe to have this immutable
    //stuff, we can also indeeed use a s.clear before immutable ref
    //s.clear();//gives error,as we are using are creating a mutable refrence when a readable
    //refrence exists
    println!("{}",slice);
}
