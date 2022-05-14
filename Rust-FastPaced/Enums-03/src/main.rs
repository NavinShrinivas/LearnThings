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
}


fn main() {
    println!("Hello, world!");
    let new_day = DayType::Weekday(Day{
        day_in_words : String::from("Tuesday"),
        date:12,
        month:21,
        year:2022
    });
    new_day.print_pretty();
}
