use std::collections;

//hash maps and error handling-1
fn main() {
    println!("Hello, world!");
    let mut first_map = collections::HashMap::new();
    first_map.insert(String::from("a"),1);
    first_map.insert(String::from("b"),3);
    let vec1= ["hhello","bye"];
    let vec2 = [1,2];

    let second_map : collections::HashMap<_,_>= vec1.iter().zip(vec2.iter()).collect();
    for i in second_map.iter(){
        println!("{:?}",i);
    }

    let s1 : String = String::from("haii");
    let mut s2 = String::from("UwU");
    let mut map3 = collections::HashMap::new();
        let mutref = &mut s2;
        map3.insert(&s1,3);
    map3.insert(&s2,4);

    /*
     * println!("{:?}",map3.get(&s2));//not allowed a mutable reference to the same string exists
     *println!("{:?}",map3.get(&String::from("haii"))); //also not allowed lamo
     */
    //the above tow are not aallowed when the mutable ref is not withing a block

    /*
         *mutref.drop() //illieagal imo
         */
    println!("{:?}",map3.get(&String::from("UwU")));
    let _int : i32 = 34;
    //println!("{:?}",map3.get(&_int)) //will give error
    for (key,value) in map3.iter(){
        println!("{} : {}",key,value);
    }

    let key = String::from("hello world");
    map3.entry(&key).or_insert(3456);
    /*
     *let mref = map3.entry(&String::from("hello world")).or_insert(0);
     **mref += 9;
     */
    //the above doesnt work as the string that is put in enrty is instantly dropped making mref
    //refer to something that doesnt exist 
    let mref = map3.entry(&key).or_insert(0);
    *mref += 9;
    println!("{:?}",map3.get(&key))
}

