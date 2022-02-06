# Module 13 : Hash-Maps and Error handling 


## Hash Maps 

Just like most thing in the book, the basics of the Hash Maps API are the only things that are covered,
look into the documentation for more.

Hash maps in Rust are similar to that on C++, they are not like the ones that you implement in your
data structures course.

Like String and Vec, Hash maps also store the values in heap storage.

Like vectors all the keys must be of one type and all values of one type, homogeneous.
Hash maps can be created from 2 vectors using the zip function along with their iterators and collect.


For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid

If there exists a mutable reference to any of the following members of the map the entire map becomes unredable, the only way
to have mutable refs is BLOCKS as seen [here](https://stackoverflow.com/questions/35765440/what-are-the-options-to-end-a-mutable-borrow-in-rust).
