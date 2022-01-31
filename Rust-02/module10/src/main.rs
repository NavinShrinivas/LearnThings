#[derive(PartialEq)]
enum IpKind {
    V4,
    V6,
}

enum IpKind2 {
    //so yes, we can have tuple data assosiated with the enum type as well.
    V4(String),
    V6(String),
} //even a struct can be the data assosiated with a type

#[derive(PartialEq)]
struct IpInfo {
    version_: IpKind,
    address_: String,
}

fn ip_builder(kind: IpKind, address_: String) -> IpInfo {
    IpInfo {
        version_: kind,
        address_,
    }
}

impl IpKind2{
    fn print_info(&self){
        match self{
            IpKind2::V4(value) =>{
                println!("Ipv 4 address");
                println!("Adress : {}",value);
            }
            /*
             *IpKind2::V6(value)=>{
             *    println!("Ipv 6 address");
             *    println!("Address : {}",value);
             *}
             */ //either exahustive or use _
            _=>{
                println!("Unhandle expection");
            }
        }
    }
}

impl IpInfo {
    fn print_info(&self) {
        /*
         *if self.version_ == IpKind::V6 {
         *    //this is actually not how u should be comparing
         *    //we will see pattern matching a little later
         *    println!("IPv 6 address");
         *    println!("Address : {}", self.address_)
         *} else {
         *    println!("IPv 4 address");
         *    println!("Address : {}", self.address_);
         *}
         */
        match self.version_{
            IpKind::V4 =>{
                println!("IPv 4 address");
                println!("Address : {}", self.address_);
            }
            IpKind::V6 => {
                println!("IPv 6 address");
                println!("Address : {}",self.address_);
            }
         }
    }
}

enum about_person{
    person(String,i32),
    None
}

fn create_person(name : Option<String>,age : Option<i32>)->about_person{
    match name{
        Option::Some(name)=>{
            match age {
                Option::Some(age)=>{
                    about_person::person(name,age)
                }
                None =>{
                    println!("You haven't provided a needed field!");
                    about_person::None
                }
            }
        }
        None => {
            println!("You haven't provided a needed field!");
            about_person::None
        }
    }
}


fn main() {
    println!("Hello, world!");
    let ipaddr1: IpInfo = ip_builder(IpKind::V4, String::from("192.168.1.11"));
    ipaddr1.print_info();
    let ipaddr2: IpKind2 = IpKind2::V6(String::from("::1"));
    ipaddr2.print_info();

    let _first_option = Option::Some(String::from("Hello world")); //allowed
    //let second_options = None; //not allowed
    let _third_option: Option<i32> = None; //alowed

    let person1 = create_person(Option::Some(String::from("Navin")),Option::Some(19));
    let person2 = create_person(Option::Some(String::from("hello")),Option::None);

    //if let Some(name) = {person1;}//Im guessing this will "move" the string
    //I thought the previous might move the string, but Rust doesn't even let us do it!
    if let about_person::person(name,age)= person1{
        println!("{} age is {}",name,age);
    }else{
        println!("Undefined Error!");
    }
}
