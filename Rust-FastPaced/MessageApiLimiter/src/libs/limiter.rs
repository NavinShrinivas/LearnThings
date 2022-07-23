use crate::Message;


pub struct UsageLimiter<'a,T : Message>{
    messager : &'a T,
    value : usize,
    limit : usize
}

impl<'a,T> UsageLimiter <'a,T> 
    where T : Message
{
    pub fn new(messager : &'a T,limit : usize) -> UsageLimiter<'a,T>{
        UsageLimiter { messager, value: 0 as usize, limit }
    }

    pub fn increment_and_check_usage_limit(self : &mut Self) -> bool{
        self.value = self.value+1;
        let percentage = self.value as f64 / self.limit as f64;

        if percentage >= 1.0{
            self.messager.push(String::from("Error! You are over your quota"));
            false
        }else if percentage >= 0.90{
            self.messager.push(String::from("Last Warning : You have used up 90% of your quota"));
            true

        }else if percentage >= 0.75{
            self.messager.push(String::from("You have used up 75% of your quota"));
            true
        }else{
            true
        }
    }
}


