use std::fmt;

enum RetMsg{
    Fail(i32),   
    Success(i32)
}
fn some_fn(x : i32 , y : i32) -> (RetMsg,Option<i32>){
    if x==3{
        (RetMsg::Fail(3),None)

    }
    else{
        (RetMsg::Success(x),Some(x+y))
    }
}
/*
 *impl<i32 : fmt::Display> fmt::Display for Option<i32>{
 *    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
 *        match *self{
 *            Some(i)=>write!(f,"{}",i),
 *            None => write!(f,"{}",String::from("null"))
 *        }
 *    }
 *}
 *As you can see,
 *you can't implement a trait you didn't write for a type you didn't write. 
 *This is part of what's known as "coherence" and exists to prevent really 
 *weird things like linking against a library suddenly causing unrelated
 *parts of your program to change behaviour.
 */


/*
 * impl<i32 : fmt::Display> fmt::Display for Option<i32>{
 *     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
 *        match *self{
 *            Some(i)=>write!(f,"{}",i),
 *            None => write!(f,"{}",String::from("null"))
 *        }
 *    }
 *}
 */



fn RetMsg_Match(en : RetMsg , val : Option<i32>){
    match en{
        RetMsg::Fail(errno) => println!("Function failed , errno : {}!",errno),
        RetMsg::Success(exit_status) => println!("Function Success ,  exit status : {} , function return : {:?}!",exit_status,val)
    } 

}
fn main() {
    let (arg1 , arg2) = (10,5); 
    let (arg3 , arg4) = (3,5);
    let (retval1,val1) = some_fn(arg1, arg2);
    let (retval2, val2) = some_fn(arg3 , arg4);
    RetMsg_Match(retval1,val1);
    RetMsg_Match(retval2,val2);
}
