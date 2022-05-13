//This section is all about structs

//Basic def syntax for enum in rust :
#[derive(Debug)]
enum MessageType{
File,
Message,
}


//Basic defining syntax for a struct : 
#[derive(Debug)]
struct Message{
from: String,
to: String,
content: Vec<u8>, //Vector for bytes
typ: MessageType
}

impl Message{
fn print_pretty(&self){
    println!("Message in storage : {:#?}",self);
}
fn is_same_sender(self: &Self,msg1: &Message) -> bool{
    if self.from == msg1.from{
        true
    }else{
        false
        }
    }
    fn content_size_in_bytes(self: &Self)->i32{
        self.content.len().try_into().unwrap()

    }
}

fn main() {
    println!("Hello, world!");
    let msg1 = Message{
        from: String::from("Navin"),
        to: String::from("User2"),
        content: vec![72,101,108,108,111,44,32,87,111,114,108,100,33],
        typ: MessageType::Message, 
    };
    println!("Messag was sent from {:?} to {:?} with content : {:?}",msg1.from,msg1.to,std::str::from_utf8(&msg1.content).unwrap());
    let mut msg2 = Message{
        from: String::from("Navin"),
        to: String::from("User2"),
        content: vec![72,101,108,108,111,44,32,87,111,114,108,100,33],
        typ: MessageType::Message, 
    };
    msg2.to = String::from(msg2.from.clone());
    println!("Message was sent from {:?} to {:?} with content : {:?}",msg2.from,msg2.to,std::str::from_utf8(&msg2.content).unwrap());
    let msg3 = Message{
        typ: MessageType::File,
        ..msg1 //NOTE : All values of msg1 that do not have copy trait are moved to msg3
    };
    //println!("Messag was sent from {:?} to {:?} with content : {:?}",msg1.from,msg1.to,std::str::from_utf8(&msg1.content).unwrap());//Causes error
    msg3.print_pretty();
    if msg3.is_same_sender(&msg2){
        println!("Same sender!");
    }
    println!("Size of content in msg3 : {}bytes",msg3.content_size_in_bytes());
}
