use text_io::read;
use std::io;
use std::io::prelude::*;

fn reversetuple(pair : (i32,i32))->(i32 , i32){
    (pair.1,pair.0)
}

fn print_arr(arr : &[i32]){
    println!("Values of array : ");
    for i in arr{ //arrays are iterrable in rust too
        print!("{} ",i);
        io::stdout().flush();
    }
    println!("End of Array.");
}

fn main() {

   // println!("Hello world!");
   
   //ok, sooo tuples , these are makde using () and can hold any number of variables
   //of any type
   // #[derive(Debug)]
    let new_tuple = (123 , 345 , "a string" , 34.65);
    println!("{:?} is the tuple",new_tuple);//debug print for entire tuple
    //can be acessed in memb using . dot operator
    
    //tuple of tuples is very possible
    //now lets look into tuples as a parameter , we will try and ocme back to 
    //functions a little later
    let another_tuple = (32 ,64); 
    println!("{:?} is the tuple {:?} is the reverse of the tuple",another_tuple , reversetuple(another_tuple));
    //tuples can be broken down and joint : i.e : 
    let a="A string";
    let number : i32 =234;
    let joint_tuple = (a,number);
    let (brokenval1 , brokenval2) = joint_tuple;
    println!("{:?} is the tuple with members of `{}` `{}`" , joint_tuple , brokenval1 , brokenval2);
    let arr1 : [i32 ; 5] = [1,2,3,4,5];//i can make a varible telling the length of array , use vectors
    //i.e let arr : [i32 , n] = [...] will give an error , for this we need vec or Box as seen in
    //some previous examples
    let mut arr2 : [i32 ; 10] = [0;10]; //all 10 values are 0 ,I can not do 0;7
    arr2[9]=19;
    println!("Values of 2nd array : ");
    for i in arr2{ //arrays are iterrable in rust too
        print!("{} ",i);
        io::stdout().flush();
    }
    println!("");
    println!("Values of 1st array : ");
    for i in (0..arr1.len()){ //arrays are iterrable in rust too
        print!("{} ",arr1[i]);
        io::stdout().flush();
    }
    //an example for enumerate : 
    for (i,j) in (5..10).enumerate(){
        println!("i : {} , j:{}",i,j);
    }
    println!("");
    //now lets talk about slices , slices are simply another "view" of the array
    print_arr(&arr1);
    print_arr(&arr1[1..4]); //full slice is from 0..5
    print_arr(&arr1[0..5]);
    //also those & are must for passing arrays , as cargo expect the size to be known compile time
    //,hence simply passing address makes it simple
    //before we end with arrays lets look into few implicit ibuilt functions
    println!("size of array is : {}",std::mem::size_of_val(&arr1));
    println!("Elements in the array : {}",arr1.len());

    
    // let mut n = String::new();
    //print!("Enter number of elements for array : ");
    //io::stdout().flush();
    //io::stdin().read_line(&mut n);
    //const len : i32 = n.trim().parse::<i32>().expect("Please enter an number");
    //this method i am not using , instead , lets start using an easier crate
    let new_string : String = read!("{}\n");
    println!("Entered sentence is : {}",new_string);
   // let n : i32 = read!("The size of vector :{} "); these words after and before only works like
   // paceholder , if the sepcific placeholder is not met , it kust leads to panic

    print!("Enter a number : ");
    io::stdout().flush();
    let new_int : i32 = read!(); //same as read!("{}")
    println!("Entered number is : {}",new_int);

    let mut new_vec : Vec<i32>=Vec::new();
    let mut exit_flag = false;
    while !exit_flag{
        println!("-----------------");
        println!("1.Push Element");
        println!("2.Pop Element");
        println!("3.Display vector");
        println!("4.Exit");
        print!("Enter choice : ");
        io::stdout().flush();
        let mut choice : i32 = read!();
        if choice==1{
            print!("Enter value to be pushed to vector : ");
            io::stdout().flush();
            new_vec.push(read!());
        }
        else if choice==2{
            println!("Value of element popped : {:?}",new_vec.pop());
        }
        else if choice==3{
            println!("Vector : {:?}",new_vec);
        }
        else{
            exit_flag=true;
        }

   }
}
    
