fn main() {
    println!("Hello, world!");
    let vector = vec![2,7,3,6,4,2,1];
    let vec_iter = vector.iter(); 

    for i in vec_iter{ //This operation moves vec_iter, we are inheritly calling into_iter()
        println!("{}",i);
    }
    //println!("{}",vec_iter.next().unwrap()); //Due to which this doesnt work
    let mut new_vec_iter = vector.iter(); //Has to be mut cus when calling next we are modifying the current valu in the iter
    println!("{:?} {:?} {:?}",new_vec_iter.next(),new_vec_iter.next(),new_vec_iter.next()); //Usage of next method
   

    //Example for consumers and adapter : 
    let test_vec = vec![1,5,5,5,5,5,5];
    let mut test_vec_iter = test_vec.iter();
    println!("{}",test_vec_iter.next().unwrap());
    let sum : i32 = test_vec_iter.sum();
    println!("Sum without 0 index : {}",sum);
    //what happens if we use map on a itetator that has been supposedly consumed entirely by sum()? 
    //let sum2 : i32 = test_vec_iter.map(|x| x+1).sum(); //Funnny thing, we simply cant use a iter
    //that has been moved, which sum() does
    let mut test_vec_iter2 = test_vec.iter();
    test_vec_iter2.next();
    let sum2 : i32 = test_vec_iter2.map(|x| x+1).sum(); 
    println!("Sum from map consuming a closure as well : {}",sum2);
    //Rust's filter function works very similar to pythons : 
    let test_vec2 = vec![1,2,3,4,5,6,7,8,9,10,5];
    let multiples_of_5 : Vec<_> = test_vec2.iter().filter(|x| *x%5 == 0).collect();
    println!("{:#?}",multiples_of_5);
}
