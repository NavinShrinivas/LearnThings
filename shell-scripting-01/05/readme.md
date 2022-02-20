# Test

## [

The comparisons we did inside [ in the while loop, that was supposedly a test!
What does this mean? that means [ is like any command [like ls and read] and like always
do, we put spaces around all our commands.

this explains the first set of spaces from the previous example.

### Operations using test

Test can do various operations on different types, but there are much more opertators than what 
have been used in the main.sh examples.
Few of these opertators are : 
> Note : To my testing, none of these operators in "test" really care about the type, kinda 
like js. If the true types of data preset within the variables are not same, It will throw error
Like we can compare 1 and "abc" but we can compare 1 and "1".

- = : if two variables are equal 
- -eq : same as above
- -lt : if less than 
- -gt : if greater than
- -le : if less than or equal to 
- -ge : if greater than or equal to
- -n : if not zero length [works on both integers and string, exaple given]
- -f : if he given path in the variable is a valid path
- -x : if the given path in var is a execuatable file, i.e the executable bit is set.

From resources : 

 -a, -e (both meaning "file exists"), -S (file is a Socket), -nt (file is newer than),
 -ot (file is older than), -ef (paths refer to the same file) and -O
 (file is owned by the user running the test) are not available in the traditional Bourne shell


## If then else fi - elif

Examples for these have been written down in main.sh

### neater if else

If you have worked with other programming languages, you have definetly written conditional 
statements using unary operators, this also very very similar.

[ ... ] && #IFCONDITIONEXEC || #ELSECONDITIONEXEC

## Few tricks to make the shell script look noice

";" can be used to combine two lines. Example : if and then must come in two lines or we can do
if [ ... ]; then 

"\" can be used to tell the script that this is not the end of this line, meaning we can write 
a single line as two.

## Excercise

Write a program to get infinite number of input [one at a time] and check if every char of ther 
input is a number.

With given what was thought so far, this will definetly be hard. Use google to the fullest 
extent. good luck!

HINT : here is a starting place, when you route outputs to a null char device "/dev/null", 
its output will end up in a special assigned variable called "$?"

HINT : To all thos who said, null device will simply eat up all streams, you are right. But you
never told what it returns. You need that here.To make not so easy,here is the null device source
code from linux source.Interpret it.

```c
static ssize_t write_null(struct file *file, const char __user *buf,
              size_t count, loff_t *ppos)
{
    return count;
}
```

HINT [Only if you couldn't figure it out] : /dev/null returns 0 if any grep was met, else 1.


With that we come the end of Module 05, it was heavy.But really fun at the same time.Hope to see 
you in 06!
