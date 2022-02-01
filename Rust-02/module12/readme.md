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
