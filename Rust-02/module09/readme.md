# module 09

Struct in Rust are quite powerful, in other languages sructs are simply user definded data type,
but in other languages user defined data types can not have atrtibutes and methods of its own.

Rust allows us to do this with in a very safe and wonderfull way. Here we see the true power of
Rust being rebuilt as a rogramming language from scratch.

Rust does not allow specific proerpties of structs to be mut.

There are spcecial things for building a struct, those are : 
- init shorthand
- update syntax.

## Methods 

are just like functions, can have parameters, arguments and return values...just that they are 
assosiated to the struct. This makes structs in Rust studpidly powerful.

soooo, I kinda called this a while back, here the formal explaination : 

## Where’s the -> Operator?

In C and C++, two different operators are used for calling methods: you use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().

Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.
