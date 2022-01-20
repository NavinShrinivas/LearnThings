
fn retlen(s : &mut String)->usize{
    //s.push_str("hahaha,ref go brr");//gives error if not mut
    s.push_str("hhahaha");
    s.len()
}


/*
 *fn dropper ()-> &String{
 *    let s = String::from("hello");
 *
 *    &s
 *}//s is dropped here, yet we are passing a readable reference to something else,hence this a error
 *
 */
//to solve the above stuff, we nee life time specifer

fn main() {
    println!("Hello, world!");
    let mut string = String::from("Hello world");
    //println!("Length of {} is {}",string,retlen(&mut string));
    //above gave error as we first we use immutable string then make a mutable ref
    let len : usize;
    {
        let r = &mut string;
        len = retlen(r)
    }
    println!("Length of {} is {}",string,len);
    let r1 = &string;
    println!("{}",r1);
    println!("{}",r1);
    let r2 = &mut string;
    println!("{}",r2);
    //println!("{}",r1); //gives error, so in theory a readable reference should not be used after
    //mutable reference
    //So the ONLY way I can read that variable or deref is to drop the mutable reference.
}
