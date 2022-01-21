
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


fn main() {
    println!("Hello, world!");
    let s : String = String::from("hwllo world");
    let space = oldwordfn(&s);
    //say we did s.clear herem space would still have a value that makes sense to previous value of
    //string, this is tedious to track , hence slices.
    //slices are redable refrence only
    let slice = newwordfn(&s);
    //s.clear();//gives error,as we are using are creating a mutable refrence when a readable
    //refrence exists
}
