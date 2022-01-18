use std::io::{self, prelude::*};
use std::process::exit;
use rand::Rng;

fn nfibb(n: i32) -> i32 {
    let mut f = 1;
    let mut s = 1;
    if n == 1 || n == 2 {
        f
    } else {
        for _i in 2..n {
            let t = s;
            s = s + f;
            f = t;
        }
        s
    }
}

fn randnum()->i32{
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

fn cel_2_faren(a:f64)->f64{
    (a*(9.0/5.0))+32.0
}

fn faren_2_cel(a:f64)->f64{
    (a-32.0)*(5.0/9.0)
}

fn main() {
    loop {
        println!("Hello, USER. Welcome to this simulation.");
        println!("----------MENU----------");
        println!("1.Convert farenheit to celsius.");
        println!("2.Convert celsius to farenheit.");
        println!("3.Compute n'th fibbonacci number.");
        println!("4.Compute a number divisible by 6.");
        println!("5.Exit.");
        print!("Enter your option : ");
        io::stdout()
            .flush()
            .expect("Something wen wrong on our side.");
        let mut option: String = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Something went wrong with readin your input");
        let mat: i32 = match option.trim().to_string().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Stop trying to break me");
                continue;
            }
        };
        match mat {
            1 => {
                let mut input = String::new();
                print!("Enter temprature in farenheit : ");
                io::stdout().flush().expect("Something went wrong in printing.");
                io::stdin().read_line(&mut input).expect("Something went wrong readin input");
                let faren : f64 = match input.trim().parse(){
                    Ok(num) => num,
                    Err(_) => { 
                        println!("Stop tyring to break me :)");
                        continue;
                    },                
                };
                println!("Given temprature in cel : {}",faren_2_cel(faren));

            }
            2 => {
                let mut input = String::new();
                print!("Enter temprature in celcius : ");
                io::stdout().flush().expect("Something went wrong in printing.");
                io::stdin().read_line(&mut input).expect("Something went wrong readin input");
                let cel : f64 = match input.trim().parse(){
                    Ok(num) => num,
                    Err(_) => { 
                        println!("Stop tyring to break me :)");
                        continue;
                    },                
                };
                println!("Given temprature in farenheit : {}",cel_2_faren(cel));
            },
            3 => {
                let mut input: String = String::new();
                print!("Enter a number to compute it from fibbonacci series : ");
                io::stdout()
                    .flush()
                    .expect("Something went wrong on our side, while printing");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Something went wrong on our side when reading last input");
                let n: i32 = match input.to_string().trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Stop trying to break me.");
                        continue;
                    }
                };
                println!("The {}(th/nd/rd) fibbonacci element is : {}", n, nfibb(n));
            },
            4 => {
                let ans = loop {
                    let r =  randnum();
                    if r%3 == 0 && r%2 == 0{
                        break r;
                    }
                };
                println!("Random divisible by 6 number : {}",ans);
            }
            5 => exit(0),
            _ => println!("Something went wrong on our side."),
        }
    }
}
