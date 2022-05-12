## RESOURCES : 

There are some of the resources I found VERRRYYYYY useful when learning rust the incredible language!

- [Module Systems](https://www.sheshbabu.com/posts/rust-module-system/)


# My Rust cheat sheet : 

## Vectors and slices

- let mut vec = Vec::new();
- vec.push(1) //Only a single value
- let mut vec2 = vec!['H','E','L','L','O'];
- vec.len()
- vec[2] //indexble
- vec.get(2) //same as indexing
- let vec_slice = &vec[2..5]; //Slices are ALWAYS refrences, the last index is not included


## Tuples  

- let new_tup = ("name_str", 32);
- let (name,age) =  new_tup //destructuring
- new_tup.1 //Dot notation for indexing, indexing start from 0
- Only has debug print trait implemented
