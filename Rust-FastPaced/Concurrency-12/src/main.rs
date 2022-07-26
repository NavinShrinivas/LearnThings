use std::thread;
use std::time;
use std::collections::HashMap;
use std::sync::mpsc;
use rand::prelude::*;
use std::sync::{Mutex, Arc};

fn main() {

    //Basic example
    let thread_func = ||{
        for i in 0..30{
            thread::sleep(time::Duration::from_millis(2));
            println!("{} from spwaned thread",i);
        }
    };

    let handle = thread::spawn(thread_func);
    for i in 0..20{
        thread::sleep(time::Duration::from_millis(2));
        println!("{} from main thread",i);
    }
    handle.join().unwrap();


    let mut test_hash : HashMap<_,_> = HashMap::new();
    test_hash.entry(String::from("Hello")).or_insert(0);

    let thread_fn_2 = move ||{
        let res = test_hash.entry(String::from("Hello")).or_insert(0);
        *res +=1;
        println!("modded value : {}",res);
    };
    let thread_handle_2 = thread::spawn(thread_fn_2);
    thread_handle_2.join().unwrap();
    //println!("{:?}",test_hash); //wont work

    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let test_str = String::from("hi deer");
        tx.send(test_str).unwrap();
        drop(tx);
    });
    println!("got : {}",rx.recv().unwrap()); //recv is a blocking function


    let(tx2,rx2) = mpsc::channel();
    let sender = move ||{
        let mut generator = rand::thread_rng();
        for _i in 0..5{
            tx2.send(generator.gen_range(0.0..10000.0)).unwrap();     
            thread::sleep(time::Duration::from_secs(1));
        }
    };
    let reciver = move ||{
        loop{
            for item in &rx2 {
                println!("Got  : {:?}",item);
            }
        }
    };
    let sender_handle = thread::spawn(sender);
    thread::spawn(reciver);
    sender_handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    let first_mutex =  Mutex::new(String::from("Hello worlddd"));
    println!("before modded through an mutex : {:?}",first_mutex);
    let mut refrence = first_mutex.lock().unwrap();
    *refrence = format!("{} {}",refrence,&String::from(" Another stringgggg"));
    println!("aqquired lock : {:?} ",first_mutex);
    drop(refrence); //Dropping the MutexGuard manually, not a nessacity.
    println!("after dropping aqquired guard : {:?} ",first_mutex);

    //Arc
    //
   }
