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
}
