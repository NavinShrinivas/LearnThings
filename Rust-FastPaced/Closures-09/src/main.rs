use std::thread::sleep;
use core::time::Duration;
use std::collections::HashMap;

use std::cmp::Eq;
use std::hash::Hash;

struct Cacher<T,V>{
    calculation : T,
    value : Option<V>
}

impl<T> Cacher<T,u32>
where
    T : Fn(u32) -> u32,
    //Note Fn is a trait, fn is the usual keyword
    {
        fn new(calculation : T) -> Cacher<T,u32>{
            Cacher { calculation, value: None }
        }

        fn get_value(&mut self,args : u32) -> Option<u32>{
            match self.value{
                None => {
                    let res = ( self.calculation )(args);
                    self.value = Some(res);
                    return self.value;
                }
                _ => {
                    return self.value;
                }
            }
        }
    }

struct BetterCacher<T,V>{
    calculation : T,
    value : HashMap<V,V> //Hashmaps surround value in Option either way
}

impl<T,V> BetterCacher<T,V>
where
    T : Fn(V) -> V,
    V : Eq + Hash + Copy //This entire thing fails if the generic doesnt have copy trait
{
    fn new(calculation : T) -> BetterCacher<T, V>{
        BetterCacher { calculation, value: HashMap::new() }
    }

    fn get_or_generate_value(&mut self, arg : V) -> Option<V>{
        match self.value.get(&arg){
            None => {
                let calculated_value = ( self.calculation )(arg);
                self.value.insert(arg,calculated_value);
                return Some(calculated_value);
            },
            Some(key_value) => {
                return Some(*key_value);
            }
        }
    }
}

fn main() {
    let first_closure = |var : String| {
        sleep(Duration::from_secs(2));
        println!("{}",var);
    };

    let second_closure = |num|{
        //something here
        println!("{}",num);
    };
    second_closure(String::from("Hello"));
    //second_closure(3); //Doesn't work
    let third_closure = |var|{
        println!("Printing var : {}",var);
        return var+3;
    };
    let mut state_things = Cacher::new(third_closure); //Ends up giving var a concrete value of u32 Cus of which:
                                                       //third_closure(String::from("Hello")); //Wont work!
    let res_one = state_things.get_value(100).unwrap(); //Saves the value plus returns it
    println!("{}",res_one);
    let res_two = state_things.get_value(110).unwrap(); //Note tht function is not called second time and neither is the value changed.
    println!("{}",res_two);


    let fourth_closure = |var|{
        println!("calculating.... res {:#?}",var);
        return var;
    };
    let mut state_things2 = BetterCacher::new(fourth_closure);
    let res_one = state_things2.get_or_generate_value(100).unwrap(); //Saves the value plus returns it
    println!("{}",res_one); //Will also print "calculating..."
    let res_two = state_things2.get_or_generate_value(110).unwrap(); //New value is generated and is stored.
    println!("{}",res_two); //Will also print "calculating..." 
    println!("{} Stored value retrived",state_things2.get_or_generate_value(110).unwrap()); //Wont print "calculating..." cus already stored.



    //Unlike functions, closure can also capture the scope of variables around them, like so:
    let str1 : String = String::from("Navin");

    let closure_capture = |last_name| -> String{
        return format!("{} {}",str1,last_name);
    };

    println!("Full name : {} ",closure_capture(String::from("Shrinivas")));
}
