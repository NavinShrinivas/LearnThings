pub mod utils;
pub mod first_mod {
    pub fn helloworld() {
        println!("Hello world for the second time!");
    }
    pub fn byeworld() {
        helloworld();
    }
}
