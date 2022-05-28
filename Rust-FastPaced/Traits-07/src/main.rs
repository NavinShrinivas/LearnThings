use std::fmt::Debug;

#[derive(Debug)]
struct Coords<T,U>{
    x:T,
    y:U,
}

struct Details<T,U>{
    age : U,
    id : T
}

pub trait RealCoords{
    fn dist_from_origin(self:&Self)->f32; //No default
}


impl RealCoords for Coords<f32,f32>{ //Trait RealCoord is only implemented when T and U are f32
    fn dist_from_origin(self:&Self) ->f32 {
        return f32::from((self.x.powi(2)+self.y.powi(2)).sqrt())    
    }
}

trait PrettyPrint {
    fn print_pretty(&self){ //with default
        println!("No specific impl of PrettyPrint traitr :(");
    }
}

impl<T,U> PrettyPrint for Coords<T,U>
 where T: Debug,
       U:Debug
{ //Traits bounds with better syntax
    fn print_pretty(&self) {
        println!("( {:#?},{:#?} )",self.x,self.y);
    }
}

impl<T,U> PrettyPrint for Details<T,U>{} //Fills in default


fn return_type_with_printprettty_trait(age : i32 , id : i32) -> impl PrettyPrint{
    Details{
        age,
        id
    }
}


use std::cmp::PartialOrd;
fn largest<T:PartialOrd + Copy>(list :&[T]) -> T{
    let mut largest = list[0];

    for &items in list{
        if items>largest{
            largest = items;
        }
    }

    largest
}

fn main() {
    println!("Hello, world!");
    let coord1 = Coords{x:2.3,y:3.4};
    let coord2 = Coords{x:3,y:4};
    println!("distance : {}",coord1.dist_from_origin());
    coord1.print_pretty();
    //println!("distance : {}",coord2.dist_from_origin()); //doesnt work
    coord2.print_pretty();
    let deet1 = Details{ age:"32",id:"237" };
    deet1.print_pretty();
    let deet2 = return_type_with_printprettty_trait(32,1234);

    let list1 = vec![1,3,65,2,1];
    let list2 = vec!["A","B","L","C"];
    println!("Largest of list 1 : {}",largest(&list1));
    println!("Largest of list 2 : {}",largest(&list2));

}
