use crate::routes;


pub fn print_234(){
    println!("234");
    routes::second_route::print_something("Called from module1");
}
