// iterators4.rs

// I AM DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return, success
    // Try not to use:
    // - imperative style loops (for, while), success
    // - additional variables, failed
    // For an extra challenge, don't use:
    // - recursion, success
    // Execute `rustlings hint iterators4` for hints.
    /*
     *let mut res : u64 = 1;
     *let mut copy = num;
     *loop{
     *    if copy == 0{
     *        break;
     *    }else{
     *        res*=copy;
     *        copy-=1;
     *    }
     *}
     *res
     */
    let mut x : u64 = 1;
    (1..num+1).collect::<Vec<u64>>().iter().map(|u|{
        x*=u;
        x
    }).collect::<Vec<u64>>().pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
