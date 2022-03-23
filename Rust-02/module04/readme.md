# Module 04

## Rust by design
Rust is by design a langage made to run into error...Errors in Rust in NO way makes you a bad
programmer. To quote the book : 

`
Compiler errors can be frustrating, but really they only mean your program isn’t safely doing what you want it to do yet; they do not mean that you’re not a good programmer! Experienced Rustaceans still get compiler errors.`

## Mut, Const,overshadowing

In previous module in line 36, we are reusing the old variable. When this happens, we say the 
original variable is being shadowed by the newwer one.

a mut is different from overshadowing, in overshadowing one can change the the type of variable as 
well. And ofc we can do as many operations we want on the variable and still keep the variable
immutable.

Alsooo, constants cant be overshadowed. Constants and static must have a fixed type def.

## Integer overflows in rust

In debug mode rust checks for overflows,in release builds tho..it doesn't. But anyhow...Rust 
handles overflows by "2's complement" method.In non fancy words, it flows back to lowest possible
value.

Integers can be represented in many forms and can be of many types, to see these types follow the
link at the bottom of this file.

## Charecters

Back to the days of C++, charecters are represented by single quotes.Unlike C, Rust's chars are of 
4 bytes in size [In c is 1 byte, this can only rep ASCII], this allow Rust chars to rep more than 
just ASCII, things like emojis, chinese...Can be represented rightfully by Rust chars.But this 
leads to a counter intuitive work of char rep, this will further be taken up elsewhere.

## Tuples

destructuting is allowed using tuples. Members in tuples are accessed using .index number

## Arrays

Arrays in Rust are rather strict [just like C++], Fixed length, homogeneuous type.Arrays allocate
space in the stack instead of heaps, well look into this soon.

Type annotation is strongly recommended, I'll be getting into the practice of doing this.
to annotate arrays : let var_name : [type;no. of elements]

this same syntax can be used to fill up the array as well, let var_name = [3;5], fills up with 5
3's

What happens when you try and access a value out of bound, if it is being done during runtime.Rust
is very safe and will not give you a garbage value [Stares at C], if it finds out of bound at 
compile, it wont even let you compile.

[Official Docs]( https://doc.rust-lang.org/book/ch03-02-data-types.html )
