mod libs;
use libs::limiter;
use std::cell::RefCell;


pub trait Message{
    fn push(self: &Self,new_str : String);
}
struct Messenger{
    pushed_content : RefCell<Vec<String>>
    //lets assume that whatever comes into this vector is actually pushed through the api.
}

impl Messenger{
    fn new()->Messenger{
        Messenger { pushed_content: RefCell::new(Vec::new())}
    }
}


impl Message for Messenger{
    fn push(self: &Self, new_str : String) {
        self.pushed_content.borrow_mut().push(new_str);
        //Unlike refrence that give compile error, RefCell give panic if borrow rules are
        //violoated, Like if I were to create 2 borrow mut, it would panic
        //RefCell keeps a count and increment imuts on borrow and muts value on borrow_mut
    }
}


fn main() {
    let messenger = Messenger::new();
    let mut limit_tracker =  limiter::UsageLimiter::new(&messenger,100);
    //even so basically messenger is imutable and mutable to everyone as long as they hold true to
    //the borrow rules!

    for _i in 0..100{
        if limit_tracker.increment_and_check_usage_limit(){
            messenger.push(String::from("Hello, world!"));
        }    
    }
    println!("{:?}",messenger.pushed_content.borrow_mut());
    let mut_one = messenger.pushed_content.borrow_mut();
    let mut_two = messenger.pushed_content.borrow_mut(); //will cause panic, but will compile
}
