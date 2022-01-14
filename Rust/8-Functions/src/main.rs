use std::io;
use std::io::prelude::*;
use text_io::read;

fn main() {
    //this modules lets see everything we can about functions :)
    //unlike c , there is no need of function declarations at all 
    //function can be defined anywhere in file , can be use anywhere
    
    //keyword `fn` is used to write functions followed by fn_name and arguments
    //in () ,  follwed by -> after which the return type is mentioned , see the
    //examples
    print!("Enter a number : ");
    io::stdout().flush();
    let input : i32 = read!();
    let ans = add5(input);
    println!("{}+5 = {}",input,ans);
    //when a fn does not return anything , you can either do ->() or
    //omit it all together
    print_num1(ans);
    print_num2(ans);
    print!("Fibbonaci series : ");
    for i in 1..input+1{
        print!("{} ",recurse_fibbonaci(i));
        io::stdout().flush();
    }
    println!();

}
fn add5( x : i32 ) -> i32 {
    return x+5;
}
fn print_num1(val : i32)->(){
    println!("The value is : {}",val);
}
fn print_num2(val : i32){
    println!("The value is : {}",val);
}

fn recurse_fibbonaci(val : i32)->i32{
    if val==0{
        return 0;
    }
    if val==1{
        return 1;
    }
    return recurse_fibbonaci(val-1)+recurse_fibbonaci(val-2);

    //ok there are more awesome concepts up ahead in functions , but 
    //before that i would have to get to up to speed on structs and
    //custom data types , sooo off to module 9 , will be back soon.
}
