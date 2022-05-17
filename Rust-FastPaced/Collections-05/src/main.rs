//Thhis module is all abotu inbuil collections and their important methods
//This Module can be better learnt from readm.md which I have tried to cover everything
//Kinda like the ultimate cheat sheet.
use std::io::Write;
use std::collections::HashMap; 

#[derive(Debug)]
struct Data{
    player_id:i32,
    age:i32,
    team:String,
    averagescore:f64
}


#[derive(Debug)]
enum UniversalType{
    Text(String),
    Number(i32),
    Charecter(char),
    Decimal(f64)
}

fn main() {
    println!("Hello, world!");
    //----------Vectors----------
    let mut v1 : Vec<i32> = Vec::new();
    let mut v2 = vec![2,4,7];
    v1.push(3);
    v2.push(3);
    v2.pop();
    println!("First element : {}",v1[0]); 
    println!("First element : {}",v1.get(0).unwrap()); //Safer way of fetching element.
    let mut input : String = String::new();
    std::io::stdout().write(b"Enter an index to fetch from vector 1 : ").unwrap();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    let input_i32 : usize = match input.trim().parse(){
        Ok(n) => n,
        _=>panic!("Invalid input for index")
    };
    match v1.get(input_i32){
        Some(n) => println!("Element found : {}",n),
        None => println!("Element not found at that index")
    };

    //Works : 
    let v1_ref = &v1[0];
    v1.push(2);

    //Wont work : 
    let v1_ref2 = &v1[1];
    v1.push(4);
    //println!("Printing a refrence that appeared before push : {}",v1_ref2); //Will break code

    for i in &v1{
        println!("Item from v1 : {}",i);
    }
    for i in &mut v1{
        *i=*i+50;
    }
    for i in &v1{
        println!("Item from v1 : {}",i);
    }

    let v3 = vec![UniversalType::Text(String::from("hello")),UniversalType::Number(34)];
    println!("from v3 : {:?}",v3.get(1).unwrap());

    //----------String-----------
    let str1 : &str = "Hello, World!";
    let mut string1 : String = str1.to_string();
    string1.push_str(&str1); //Take refrence of str1 thus ownership of str1 is maintained.
    println!("{}",str1);
    let string2 : String = String::from("Hello world, twice!");
    let string3 = string2 + &string1; //ownership of string2 is transffered. string1 remains the same.
    println!("{}",string3);
    let temp_str1 = String::from("5");
    let temo_str2 = String::from("7");
    let string4 : String = format!("{}\"{}\' tall",&temp_str1,&temo_str2);
    println!("Your height : {}, temo_str1 is not moved : {}",string4,temp_str1);

    //indexing string : 

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}",s); //Will not print 4 chars, prints 2 cus utf-8 encoding of 2 bytes.
    
    //----------Hash Maps----------
    
    let mut data_map1 : HashMap<String,Data> = HashMap::new();
    data_map1.insert(String::from("Player1"),Data { player_id: 237, age: 23, team: String::from("Team A"), averagescore: 34.7 });

    let players : Vec<String> = vec![String::from("Player2"),String::from("Player3")];
    let player_data : Vec<Data> = vec![Data { player_id: 238, age: 32, team: String::from("Team B"), averagescore: 35.7 },Data { player_id: 238, age: 21, team: String::from("Team A"), averagescore: 42.7 }];
    let mut data_map2 : HashMap<_,_> = players.into_iter().zip(player_data.into_iter()).collect();
    println!("{:#?}",data_map2);
    println!("{:#?}",data_map2["Player3"]);
    println!("{:#?}",data_map2.get("Player2").unwrap());

    data_map2.entry(String::from("Player2")).or_insert(Data { player_id: 400, age: 32, team: String::from("Team B"), averagescore: 35.7 }); //Wont apply
    data_map2.entry(String::from("Player1")).or_insert(Data { player_id: 237, age: 23, team: String::from("Team A"), averagescore: 34.7 }); //Will apply
    println!("{:#?}",data_map2);

    let mut hash3 : HashMap<String,i32> = HashMap::new();
    let text = "she sells sea shells at the sea shore";
    for i in text.split_whitespace(){
        let count = hash3.entry(i.to_string()).or_insert(0);
        *count+=1;
    }
    println!("{:#?}",hash3);
}
