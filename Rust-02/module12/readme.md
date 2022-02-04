# Common Collections!

Rust's standard library (although not as rich as C++) has a few collections types, data 
structures, we will be going over them now...along with their property.
Mostly :
- Vectors
- Strings
- HashMaps

## Vectors 

Vectors are stored in similar manner to String in Rust, i.e using Heap and stack. Meaning all the rules of ownership and
borrow checker applies here as well, no mutable borrows when immutable borrows are active.

Vectors can normally only store single type of data, but with use of enums we can store different types in a single 
array.

## Strings 

String are a Vector of bits in Rust and store any valid utf-8 encoded string, it has a "push_str" method which takes a
mutable reference, it also has "push" method.

Concatenation with the + operator is the tricky one, it take one String and one &str, moves the String to new String 
and uses immutable reference of &str to add value!.

But this way of concatenation is very messy when you want to start doing more complex stuff, hence we can use the
format! macro. Format macro is very similar to our println! except it returns a values back.

## Indexes is string 

Rust does not like index string, why? Unlike c arrays, Rust vectors can store all sorts of unicode chars as long as they are 
properly utf-8 encoded.

Blatantly index vectors does give an error, but you should ask, Navin what about slices?I would reply with : 

## Internal representation 

The string "hello" will simply give a length of 4, but say a more complicated string like : "Здравствуйте", you are expecting'
it's string length to be 12, but rust in-fact tells 24, why?!?!

The same problem happens with hindi written in Devanagiri script.

The explanation in the book for slices are just too beautiful, hence I am copy pasting it here simply.


## Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices. To be more specific in your indexing and indicate that you want a string slice, rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

```rs
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.

What would happen if we used &hello[0..1]? The answer: Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

```$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
       Finished dev [unoptimized + debuginfo] target(s) in 0.43s
            Running `target/debug/collections`
            thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:4:14
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace```

You should use ranges to create string slices with caution, because doing so can crash your program.

## Finally iteratintg string 

So how does one iterate over string in Rust? One uses the .chars() method in strings
