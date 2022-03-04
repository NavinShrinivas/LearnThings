# Module - 15 

## Generic types 

If I understood right from the book, generic types are polymorphism without OOPS concepts.

The first function we write using a generic type is the largest on an array irrespective of the type.
Generic Types (T) do have traits such as comparision, aritemetic, blah blah implmented. Its ofc the programmers responsbility, this work of defining traits (If your are coming from OOPS background the word "method" will resonate better).

above doesnt work, as there can be onyl one impl with one name, but it can be restricted through generic types.


You might be wondering whether there is a runtime cost when you’re using generic type parameters. The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

Stupidly beautiful rust, MAH GOD it never stops to amaze me. 

## Traits 
So we have seen how to bound impls using specific types, with what i have initially understood. We can define few methods for generics that have a particular trait.

While going through book try and relate traits and behviours (methods) to humans and it made much more sense to me.

Coming from C and C++ this traits felt stupidly similar to 

Same behviour names can be used across traits but ofc not within, We can also give behaviour traits default fns.
