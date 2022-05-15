mod module1;
mod routes;


fn main() {
    println!("Hello, world!");
    module1::print_234();
    routes::first_route::print_something("Hello, World!");
    routes::second_route::print_something("Hello, World!")
}
