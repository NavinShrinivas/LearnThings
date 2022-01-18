fn main() {
    println!("Hello, world!");

    //instead of let, we use the const keyword
    const _var: i32 = 34;
    //var=12;//gives error, constants like vars are by default immutable, simply that
    //they are always immutable and mut doesn't work on them.
    println!("About constants.");
    let var: String = String::from("Hello world");
    let var: i32 = 2304; //over shadowing
    println!("hmnmn, constanst can be overshadowed as well, {}.", var);
    let tup1 = (1, 3, 4, 5);
    let (x, y, z, a) = tup1;
    println!(
        "We also learnt about tuples, the 3rd element of the tuple is {}",
        tup1.2
    );
    let arr = ["first element", "second element"]; //altho rust can handle type annotation,
                                                   //it's a very bad practive, so for arrays :
    let arr1: [i32; 3] = [2, 3, 4];
    let arr2 = [5; 20];
    println!(
        "We also handle arrays in this module {} {} {}",
        arr1[0], arr[1], arr2[19]
    );
}
