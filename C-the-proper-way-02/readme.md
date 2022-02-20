# Learn C the proper way

What's the proper way? Going by my professor, learning things out of the bible for C langauge : 
The C Programming langauge. This is exactly what I will be doing.

## Hello world

We know the basics alredy, but you sill dont!

- what happens if you include some escape sequence that is not part of the c language...like \c?
    Nothing much, gives a warning and prints c instead. Try it out.

## First program

In this we cover : formatted print, interger approximation of floating point numbers.

- excersise :
```
print the follow farento celcius table : 
 1 -17
 20 -6
 40 4
 60 15
 80 26
 100 37
 120 48
 140 60
 160 71
 180 82
 200 93
 220 104
 240 115
 260 126
 280 137
 300 148
```
With your expertise in C this should be no problem, but do it in the way the god's of C have done
it.

The reason for multiplying by 5 and dividing by 9 instead of just multiplying by 5/9 is that in
C, as in many other languages, integer division truncates: any fractional part is discarded.
Since 5 and 9 are integers. 5/9 would be truncated to zero and so all the Celsius temperatures
would be reported as zero. 

NOTE : printf and scanf are not part of the C programming language, the are simply a standard on
the ANSI implmentation. The C language doesn't have any formal definition for output and input.

The previous excersise, the output wasn't pretty, how to make it pretty? use widths in the 
formatted printf [Implmented in code], this fills the output with space before to the specfied #.

In the first implmentation we use a kinda smort way to not getting 0 even though we have decimal
point number, this lead to a loss in accuracy, hence now implment in floating number.

