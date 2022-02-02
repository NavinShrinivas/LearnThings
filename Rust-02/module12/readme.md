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


