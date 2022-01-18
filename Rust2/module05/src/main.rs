use rand::Rng;


fn new_fn(first_arg : i32){
    println!("This is being printed from a function {}",first_arg);
    //this function is a statements
}

fn return_rand_0_100()->i32{
    let mut rng=rand::thread_rng();
    rng.gen_range(0..100)

}
//all function definitions are by default statements

fn main() {
    println!("Hello, world!");
    new_fn(1234);
    let y = {
        let x=12;
        x+1
    };
    println!("Varaible : {}",y);
    println!("Random number : {}",return_rand_0_100());
    let con : bool = true;
    let mut res = if con { 5 } else { 4 };
    println!("Values : {}",res);
    loop{
        if res < -1{
            break;
        }
        println!("Counting down : {}",res);
        res-=1;
    }
    let divis6 = loop{
        let r = return_rand_0_100();
        if r%3==0 && r%2==0{
            break r;
        }else{
            println!("Not divisible by 6, trying again");
            continue;
        }
    };
    println!("Divisibly by 6 : {}",divis6);
    let mut divis6_2 = 1;
    while divis6_2%2!=0 || divis6_2%3!=0{
        println!("Not divisible by 6, trying again");
        divis6_2=return_rand_0_100();
    }
    //.rev() works on iterators, and .. is used to create a iterator
    println!("Number divible by 6 : {}",divis6_2);
    for i in (1..5){
        println!("Countup : {}",i);
    }
    for i in (1..5).rev(){
        println!("Coutndown : {}",i);
    }
}
