# Module 07

Instead of passing(moving) the whole variable to the function, we can pass a reference of the 
string.

and as again, the diagrams in the official book is sooooo well made, please make use of them.
Rust by far has the BEST documentation I've ever seen.

In rust : Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to. DONT YOU TRY DEREF AND IMMUTING, it will blow up in your face.

We can instead, make the references also as mutable, here Rust imposes another restriction.
You can only have one "mutable reference" at a time for one variable.VERY IMP.Why did Rust do this?
Think, we have run into race conditions before, doign this...Race conditions are close to 
impossible.

Oh Oh OHHH, one more restriction, YOU CANT HAVE ONE IMMUTABLE AND MUTABLE reference at the same
time.Thus multiple immutable or 1 mutable.simple.


Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:

```rs
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point
let r3 = &mut s; // no problem
println!("{}", r3);
```

I've pretty much cleared this clearly in the main.rs file.With how Rust have implemented this shit
Race conditions are IMPOSSIBLE in my opinion, you have no clue how blown my head is now xD.

to sound smort : 
The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)

In the next module we will look at different type of references.

## Dangling pointers

Lifetime specifers FINALLLLY MAKE PROPER SENSE!!!!See the dropper function in main.rs.

We will learn how to solve these a little later, I promise.
