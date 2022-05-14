## RESOURCES : 

There are some of the resources I found VERRRYYYYY useful when learning rust the incredible language!

- [Module Systems](https://www.sheshbabu.com/posts/rust-module-system/)


# My Rust cheat sheet : 

## Vectors and slices

- let mut vec = Vec::new();
- vec.push(1) //Only a single value
- let mut vec2 = vec!['H','E','L','L','O'];
- vec.len()
- vec[2] //indexble
- vec.get(2) //same as indexing
- let vec_slice = &vec[2..5]; //Slices are ALWAYS refrences, the last index is not included


## Tuples  

- let new_tup = ("name_str", 32);
- let (name,age) =  new_tup //destructuring
- new_tup.1 //Dot notation for indexing, indexing start from 0
- Only has debug print trait implemented

## Structs

- Basic syntax for defining a struct : 
```rs
struct Message{
    from: String,
    to: String,
    content: Vec<i32>, //Vector for bytes
    typ: MessageType
}
```
- A struct can be built and is reffered to as creating and "instance", NOTE : To construct a struct you must provide all fields.
```rs
let msg1 = Message{
        from: String::from("Navin"),
        to: String::from("User2"),
        content: vec![72,101,108,108,111,44,32,87,111,114,108,100,33],
        typ: MessageType::Message,
};
```
- fields of a stuct are read using dot notation
- Entire instance must be mutable, individual fields can not be mutable by themselves.
- [Struct Update Syntax] Another instance can be created from existing ones, all new field value must be explicitly mentioned before the ..old_struct, like so : 
.
```
let _msg3 = Message{
        typ: MessageType::File,
        ..msg1 //NOTE : All values of msg1 that do not have copy trait are moved to msg3
    };
    //println!("Messag was sent from {:?} to {:?} with content : {:?}",msg1.from,msg1.to,std::str::from_utf8(&msg1.content).unwrap());//Causes error
```
Be very vary of the move happening in the "Struct Update Syntax", as the name suggests, this must be used for updating a struct not creating new ones.
- Rust also has tuples struct, where the field names are not given:
```rs
struct Color (i32,i32,i32);
struct Coord_3d (i32,i32,i32); //Can destructure and index using . notation
```

The choice if String instead for &str was  a deliberate choice above, this is cus Rust expects a struct to own all it's values. If a struct should store a reference then we need lifetime specifiers which we will see later.

- Structs by themselves do not have debug print trait, and hence needs to be derived using #[devive(Debug)]
> NOTE : Every field of the struct must have the same set of derived traits.
- Struct has have methods closely tied to themselves in Rust, These are called implmentations.

## Enums and Options : The alternative to the billion dollar mistake. How Enums tie it all together.
- Basic enum definition syntax : 
```rs
#[derive(Debug)]
struct Day{
    day_in_words: String,
    date: i32,
    month: i32,
    year: i32
}
#[derive(Debug)]
enum DayType{
    Weekday(Day),
    Weekend(Day)
}

impl DayType{
    fn print_pretty(self: &Self){
        println!("{:#?}",self);
    }
}
```
- Options :
```rs
let x: Option<i32> = Some(31);
let _y = x+34; //Gives error, cant add Option and i32
```
That is hwo rust makes sure that a null reference can never happen, Option is more like indicating that a variable can be of typer None, meaning before you can do any operation on it some check have to be done. If the variable is not of type option is can never be None. 
> Note : None is analougous to NULL is other subject, but ofc better.
- This is where we get `match` to help us, NOTE : match has to be exhaustive at all times or have _ arm. Using match with Options : 
```rs
x = match x{
    None => { 
        println!("Not initiliased");
        None
    },
    Some(x)=>{
        Some(x+31)
    }
}
```
- Using match with above enums : 
```rs
impl DayType{
    fn print_pretty(self: &Self){
        println!("{:#?}",self);
    }
    fn is_tueday(self : &Self){
        match self{
            DayType::Weekday(day)=>{
                if day.day_in_words == "tuesday"
                {
                    println!("It is tueday!");
                }else{
                    println!("It is not tuesday :(");
                }
            },
            _=>println!("It is not tuesday :(")
        } 
    }
}
```
> NOTE : the Enums-03 module is very very well written! refer it.
