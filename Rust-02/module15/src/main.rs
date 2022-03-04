struct Coords<T> {
    x: T,
    y: T,
    z: i32,
}

impl<T> Coords<T> {
    fn printf(&self) {
        println!("{}", std::any::type_name::<T>());
    }
}

impl Coords<i32> {
    /*
     *fn printf(&self) {
     *    println!("The impl for i32 has bee called");
     *}
     */
    //above doesnt work, as there can be onyl one impl with one name, but it can be restricted
    //through generic types.
    fn print32(&self) {
        println!("The impl for i32 has bee called");
}
}

/*
 *fn generic_largest<T>(list: &[T]) -> T {
 *    let mut largest = list[0];
 *
 *    for i in list {
 *        if *i > largest {
 *            //lets get to correting this error later
 *            largest = *i;
 *        }
 *    }
 *    largest
 *}
 */


// Traits : 
 

pub trait Sad{
    fn hit(&self){
        println!("Generic response of sadness");
    }
     fn cry(&self){
        println!("Generic response of sadness");
    }
     fn moody(&self){
        println!("Generic response of sadness");
    }



}

pub trait Happy{
    fn hit(&self);
    fn cry(&self);
    fn moody(&self);
}

struct Kid{
    name : String
}

struct Adult{
    name : String
}

struct OldPeople{
    name : String
}

struct Dead{
    name : String
}

impl Happy for Kid{
   fn hit(&self){
        println!("{} nenver hit people cus they happy",self.name);
    }
    fn cry(&self){
        println!("{} cry cus he/she/others upset at times",self.name);
    }
    fn moody(&self){
        println!("{} never be moody",self.name);
    }
//every fucntion in the trait must be implemented
} 


impl Sad for Adult{
    fn hit(&self){
        println!("{} hit people cus they sad :(",self.name);
    }
    fn cry(&self){
        println!("{} cry cus he/she/others Sad",self.name);
    }
    fn moody(&self){
        println!("{} always be moody",self.name);
    }
//every fucntion in the trait must be implemented
}

impl Sad for Dead{ //enuf for getting default behaviours of the trait into scope.
}






fn main() {
    println!("Hello, world!");
    let coordinate1: Coords<i32> = Coords {
        x: 12,
        y: 13,
        z: 15,
    };
    //let coordinate2 = Coords { x : 13 , y :13.5 , z : 78 }; //not allowed
    let coordinate2 = Coords {
        x: 30.5,
        y: 40.0,
        z: 45,
    };
    coordinate2.printf();
    coordinate1.printf();
    coordinate1.print32(); //works
                           //coordinate2.print32(); //doesnt work :)
    // Traits : 
    let kid1 = Kid{name : "Baby 2343".to_string()};
    let adult1 = Adult {name : "Adult 53452".to_string()};
    let _old1 = OldPeople {name : "Old person 23423".to_string()};
    let dead1 = Dead { name : String::from("Dead 2321") };

    kid1.hit();
    adult1.hit();
    //old1.hit(); //wont work
    dead1.cry();
}
