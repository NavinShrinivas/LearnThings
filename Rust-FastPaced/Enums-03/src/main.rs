//This moduel is all abotu enums and options
#[derive(Debug)]
struct Day{
    day_in_words: String,
    date: i32,
    month: i32,
    year: i32
}
#[derive(Debug)]
enum DayType{
    Weekday(Day),
    Weekend(Day)
}

impl DayType{
    fn print_pretty(self: &Self){
        println!("{:#?}",self);
    }
    fn is_tueday(self : &Self){
        match self{
            DayType::Weekday(day)=>{
                if day.day_in_words == "tuesday"
                {
                    println!("It is tueday!");
                }else{
                    println!("It is not tuesday :(");
                }
            },
            _=>println!("It is not tuesday :(")
        } 
    }
}

fn main() {
    println!("Hello, world!");
    let new_day = DayType::Weekday(Day{
        day_in_words : String::from("tuesday"),
        date:12,
        month:21,
        year:2022
    });
    let new_day2 = DayType::Weekday(Day{
        day_in_words : String::from("monday"),
        date:12,
        month:21,
        year:2022
    });
    new_day.print_pretty();
    let mut x: Option<i32> = Some(31);
    //let _y = x+34; //Gives error, cant add Option and i32
    x = match x{
        None => { 
            println!("Not initiliased");
            None
        },
        Some(x)=>{
            Some(x+31)
        }
    };
    new_day.is_tueday();
    new_day2.is_tueday();
}
