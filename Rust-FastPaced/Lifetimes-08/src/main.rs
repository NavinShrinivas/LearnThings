

fn longest1(x:&str , y:&str) -> String{
    if x.len() > y.len(){
        x.to_string()
    }else{
        y.to_string()
    }
}

/*
 *
 *Doesnt work!
 *fn longest1(x:&str , y:&str) -> &str{
 *    if x.len() > y.len(){
 *        x
 *    }else{
 *        y
 *    }
 *}
 *
 */


fn longest2<'a>(x:&'a str , y:&'a str) -> &'a str{
    //Generic life times simply tell that the returned 
    //value will live for as long as the two inputs
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn main() {
    let str1 = String::from("asdfkjhsdfjgh");
    let str2 : &str = "sdfgd";
    println!("Largest : {}",longest1(&str1,str2)); //Works just fine
    println!("Largest : {}",longest2(&str1,str2)); //Works just fine

    /*
     *Doesnt work
     *let string1 = String::from("long string is long");
     *let result;
     *{
     *    let string2 = String::from("xyz");
     *    result = longest2(string1.as_str(), string2.as_str());
     *}
     *println!("The longest string is {}", result);
     */
    //In above code 'a is replaced with concrete life time of smaller life time that is (string2)
    //but return result has a bigger lifetime and hence causes error.

    //Works : 
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest2(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    //Life time of 'a is replaced with that of string2 but return result also has same life time
    //and hence no error and vialotions.
}
