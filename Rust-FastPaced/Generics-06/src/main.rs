//function is generic over some type T that takes in a vector or type T and return a single
//instance of T
//simplyre witing thefunction wouldgivean std::cmp::PartialOrd erre kinda thing
/*
 *fn largest<T>(list : &[T]) -> T{
 *    let mut largest = list[0];
 *    for &i in list{
 *        if i > largest{
 *            largest = i;
 *        }
 *    }
 *    return largest;
 *}
 *
 */ 
//Above doesnt work, cus T doesnt have std::cmp::PartialOrd trait and cant apply > on it.

struct Pairs<T>{
    x: T,
    y: T,
}

struct PairsHetro<T,U>{
    x:T,
    y:U,
}

use std::fmt::Debug;
impl<T:Debug,U:Debug> PairsHetro<T,U>{ //These methods are available to all T and U that have Debug trait
    fn pretty_print(self:&Self){
    println!(" ({:#?},{:#?}) ",self.x,self.y);
    }
}

impl PairsHetro<i32,f32>{
    fn add2(self:&mut Self){
        self.x+=2;
        self.y+=2.0;
    }
} 

//In all trait check happen on generics going into impl and type checks happen on generics going into
//the object (struct or enum or whatev)


fn main() {
    println!("Hello, world!");
    let p1 = Pairs{ x:12,y:23 }; //Works
    //let p2 : Pairs<T> = { x:23,y:34  }; //Wont work cus main function doesnt know what <T> is
    let p2 : Pairs<i32> = Pairs{ x:23,y:34 }; //Works
    //let p3 = Pairs{ x:2.3,y:23 }; //Wont work;
    


    let p3 = PairsHetro{x:2.3,y:23}; //Works!
    let mut p4 = PairsHetro{x:32,y:3.2};
    //p3.add2(); //Does nto work
    p4.add2(); //works
    p3.pretty_print(); //Work 
    p4.pretty_print(); //Works
}
