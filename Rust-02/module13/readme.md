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

To refresh your memory, cus it's needed for hash maps : 

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


## More about Hash maps 

We use the .get method to access values from the hash maps, this method expects a reference to the key.
.iter methods are implemented for hash maps and can also be used to access stuff in hash maps.


## Updating contents in the hash maps 

### Overwriting

Simply the HashMaps::insert method will over write any existing methods

### Inserting only if the key doesn't exist 

we can achive this using .entry and .or_insert method, entry method gives a Entry enum which has impl or_insert, this implmented function checks if the key exists and if not inserts it

### Updating already existing value 

The or_insert method inserts if the key doesn't exist, but it also returns a back a "mutable" reference to the new or already existing value in the map. This is what we have to use to update an already existing value.

## hashing functions 

We have learnt hash maps in depth in our dsa course and we used to use dumb mod functions as our hash functions, very unsafe. Rust uses hash function called SipHash which is resilient to DDOS attack.


# Error handling 

Rust is very unlike other languages in this palce, it 3 diff kind of errors , recoverable and Non recoverable. To handle these Rust has Result<T, E> and !panic respectively. In this module ill deal with !panic : Non recoverable errors.

## Recoverable  Errors : !panic 

Rust programs usually "unwind", i.e they go through the entire stack and clear memory of each function they come across, this is good but its slop, languages like C simply abrupt, this change can be made 
