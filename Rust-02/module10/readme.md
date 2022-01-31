# Enums and Pattern matching

## Enums 

Enums in Rust are very unlike C, hence the C and C++ programmers have to give more heed to this
module.

Like C, enums will have "types/enumarations" possible for the datatype, but unlike C, these typescan also store respective data, this combined with pattern matching, Enums become very neat and
powerful in Rust.

## Options 

Rust doesn't have NULL quoting the inventor of NULL Tony Hoare: 
```
  I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.
```

This isnt' gone in depth in the Rust documentation, I'll dive into Options much deeper in the next module.
Simply remember, Options are just another enum with Some() and None types, all the other properties apply
to them as well.

## Back to enums, This time with match operator

using match with enums make the code seem VERY NEAT, but one thing I found awkward with match is that 
it's enum "exhaustive", meaning? You must have an arm in match for all the possible enum types, else in
gives error. Altho im sure i can simply add one _ => arm to over come this.

## Consise is let construct 

If let works a consise alternative to match operator, they are much neater and easier to look at,
but surely harder to comprehend.It works similar to tuple desructing but with enum types.

