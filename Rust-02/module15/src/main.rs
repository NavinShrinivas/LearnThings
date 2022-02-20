fn generic_largest<T>(list: &[T])-> T{
    let mut largest = list[0];

    for i in list.iter(){
        if(i > largest)
            { largest = i; }
    }
}


fn main() {
    println!("Hello, world!");
}
