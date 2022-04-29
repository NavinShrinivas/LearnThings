
fn main() {
    //OWNERSHIP
    //
    println!("Hello, world!");
    let mut st : String = String::from("Hello, world!");

    let i1 : i32 = 23;
    let i2 : i32 = i1; //Deep copy of stack allocated vriables autmatically.

    let s1 = st; //Now we have 2 variables pointing to the same value in the heap 
    let mut s2 = s1.clone();
    //s1.push_str("Hello"); //Not valid as s1 not mut
    //Image 1 of readme shows this in a nice way 
    //Now when we reach end of this function both s and s2 are trying to free the same memeory in
    //the heap, this is called double free error. To solve this Rust no longer considers s to be
    //valid!
    //println!("{:?}",s); //Gives error of s being moved. And because of this shallow copies are
    //called are "moves in rust!"
    //If we want to deep copy we use the clone method of heap allocated vars. If a type implements
    //the "Copy" trait its variables are valid even after and copy (deep and shallow are same in
    //this case)
    //If the type has "Drop" train implmented, it wont let us implement a Copy trait
    //Just like that, passing variables in and out of functions cause a move
    //Thus moving variables in and out of functions seem quite tedious, hence we have refrences to
    //the rescue


    //REFRENCES
    printer(&s1); //Second images in docs makes this clear
    println!("{}",s1); //Works
    //refrencing is called as borrowing in rust, and stuff that are borrowed by us (irl) must be
    //handled with care, same way u cant modfied borrowed vars.
    //Altho mutable borrows (refrences with mut) can be modified.
    //But these come with a HUGEEEEE restriction : YOU CAN HAVE ONLY ONE MUT REF AT ONE TIME
    //Another one, cant have mut ref when imut ref's alredy exists 
    //I mean you can have mut ref after imut ref, just that the value must not br read using the
    //imut ref when mut ref exists


    let r1 = &s2; //works 
    let r2 = &s2; //works 
    println!("{}{}",r1,r2);
    let r3 = &mut s2;  //Works as long as r1 and r2 arent used after this line.
    //println!("{}",r1); //Will give error
    let r4 = &s2; //works as long as r3 aint used after
    println!("{}",r4); //works as long as r3 aint used after 
    //println!("{}",r3); //Breaks code
    //This mess of a scope system is handler by NLL : non lexical lifetimes

    //danglings pointers are impossible in rust, the following example will show dis : 
    let want_ref = get_ref();
    




}// things are "droppped" here

fn printer(s : &String){
    println!("{}",s); //Derefned without *, we'll see how later
}

fn get_ref()->&String{
    let s : String = String::from("OwO");

    &s //this in c would have caused a danglign pointer
}//remember s is dropped here
