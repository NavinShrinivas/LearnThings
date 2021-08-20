macro_rules! print_pi{
    ($a:expr)=>{
        {
            println!("Pi is roughly {pi:.width$}",width=3,pi=$a);
        }
    };
}

fn main() {
    //i can write comments just as in c/c++
    println!("Hello, world!");
    println!("{} days",31);
    println!("{0}, this is {1}. {1}, this is {0}", "Roji", "Navin");
    //above is to show positional arguments in println

    println!("{subject},{object}",object="the lazy dog",subject="the quick brown fox",);
    //the above shows "named arguments" can be used for formatted print as well

    //all special formatting is done using a colon ":"
    //, these colons are mainly used for padding with whitespace or random characters 
    //also the keyword for padding in special formatting is "width" , ans number of width is always
    //width-1

    println!("{firstword:>width$},World",firstword="Hello",width=6);
    //the above only gives 1 space , this is because width doesn't give simply padding ,
    //it make sure your output take up the amount of width specified , i.e 5 for hello ,1 for space
    //very similar to c :

    /* char *ptr = "Hello";
    printf("%40s\n", ptr);
    That will give you 35 spaces, then the word "Hello".
    This is how you format stuff when you know how wide you want the column, but the data 
    changes (well, it's one way you can do it).
    */

    println!("{firstword:0>width$},World", firstword="Hello", width=8);//i,e 3 zeros

    //mild intro to macros and variables
    let pi=3.1415926535;
    print_pi!(pi);
}
