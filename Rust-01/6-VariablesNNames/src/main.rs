use rand::prelude::*;
//use std::mem::prelude::*; sadly thers is not prelude method in std::mem
use std::any::type_name;
fn main() {
    //if you remember , i mentioned all varibles are immutable by default in rust 
    //but u can change value of even immutable varibles using a method called "shadowing"
    //like :

    let a : i32 =10; //immutable
    let a=a*    2; //works
    let a=a+10; //work 


    //the above two lines worked only cus of the presence of `let` keyword
    println!("Value of a : {}",a);

    //lets look into constants , these are thos that are always immutable , cant be overshadowed 
    //and always have to have types annotated to them
    const b : &str = "hello world";

    let v : i32 = "234".parse().expect("Hmmmm , Not a number ?"); //observe i havent done <i32> as rust
    //is insanely smart and can figure out my output type :)
    println!("Value of parsed string to int : {}",v);


    //ahhh one of the stuff i forgot to mention in CratesNRandom module was that the rand crate can
    //generate a random number of a specific type as same as that its being assigned to
    //i.e :
    let mut rng=thread_rng(); //generator;
    let float_rand : f32 = rng.gen_range(1.0..101.0); //auto float
    let int_rand : i32 = rng.gen_range(1..101); //auto int
    if rng.gen(){ //auto boolean
        println!("The gods have decided to bless us with a True!");
    }
    println!("{} {} are the two rand numbers generated",float_rand,int_rand);


    let long_lived_var : i32 = 25;
    let mut long_mut : i32 = 67;
    {
        let short_lived_var = 35;
        let long_lived_var = 45;
        long_mut = 70;
        println!("the three variables are : {} {} {}",long_lived_var,short_lived_var,long_mut); //no errors
    }
    // println!("The short lived var : {}",short_lived_var);//error if uncommented}
    println!("The long lived var : {}",long_lived_var); //the scope of value 45 for long_var is thos {}
    //outside the scope it reverts to the old one
    //if not initialized outside {} , it leads to uninitialized errors

    println!("The mutable element : {}",long_mut); // the same does not happen with mutable elments woa!

    //simply tl;dr : shadowing is scoped !


    let mut mut_var : i32 = 34;
    // let mut_var = mut_var;
    mut_var = 40;//error if line before is uncommented

    {
        // let mut_var = mut_var;  //the docs mention something about "forzen" but i dont wanna
        //simply drop an new term if not needed
        mut_var=50;//error if previous line active
        println!("var : {}",mut_var);
    }
    println!("var : {}",mut_var);

    //-------------------------------------------TYPES-----------------------------------------------

    //using as keyword to type cast the hell out of everything
    //there is not implicite type conversion in rust , but using the as keyword we do explicit type
    //conversion

    let new_float : f64 = 4.4534435;
    // let new_int : i32 = new_float; //error
    let new_int : i32 = new_float as i32; //works
    let character : char = new_int as u8 as char;
    // let character = new_float as char; //error 
    //tl;dr : float -> int(u8) -> char
    println!("vars are : {} {} {}",new_float,new_int,character);


    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }//use these methods ver very carefully

    //anyways now lets look into the "sizeof" operator
    let integer : i32 = 34;
    println!("Size of i32 : {}",std::mem::size_of_val(&integer));     


    //-----------------------INFERENCE---------------------
    //again here we see RUST's very smart skills , a vector (Dynamic array)
    //doesnt need to know the type of elements stored during defining / declaring
    //
    //rust will automatically take the type when first element is stored

    let mut vec = Vec::new(); //we will look into these next module
    vec.push(integer);
   // println("type of vector : {}",type_name::<&vec>()); i dont see a way to do this
   // in rust
   
    //ALIASING , any type can be given a alias , but , without UpperCamelCase
    //this would again raise and error , WHY RUST WaHY!!!!??
    
    type Intergers = i64;
    let newint : Intergers = 2354345;
    println!("{}", newint);
}


