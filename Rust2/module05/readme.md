# Module 05

## Functions

These are pretty basic to start with, arguments and parameters are pretty much the same as they
are in C++.Return types are when things get reall interesting, keep a look out for them

### Expressions and Statements and how it relates to fn's

All functions by default are statements and not expression, statements are lines of code that 
do not return anything back. Supposedly assigning a variables in other languages are expression,
that is they actually return something, Rust differs in this place, assigning variables is a 
statement as it should be.

all function definitions are by default statements.

In rust you can put both expression and statments into statements.This leads to something similar
to how js handles fn, altho I am not sure if these fn type things can take in arguments like in js.

EXPRESSIONS DO NOT HAVE A SEMICOLON ENDING THEM

### Returning from functions

Quite simple returning is handled here, Results are handles laters I guess then.

## Conditional stuff

In module 3 we so immaculately and neatley use match for comparing and conditinal branching, here
we instead use if conditions [Yeah I know not as nice as match].

unlike C we can't do :
```rs
if 3{
println!("something")
}
```
Rust striclty expects a bool value,also `while true`, instead use loop.
Fancy fancy , we can use if statements in let statements.
In this statemen, if the "if" and "else" conditions assing a different data to the variable, rust 
compiler gives an error, that is sooo cool.

## Loops

You can return a value from a loop, this is often used for expressions we know might fail a few 
times.In such a case we might have to also return a value and break the loop when successfull,
here break used quite differently [See code].`break return_val`

Now ofc the above can be done using a while loop quite easily, so don't go around reinventing
the wheell when not needed.

while loops are quite basic, for loops work with iterators easily in rust. For loops are know to 
be more safer and consideres corners case, be the rustacean and use the for loop.

To non iterate, see the example.
