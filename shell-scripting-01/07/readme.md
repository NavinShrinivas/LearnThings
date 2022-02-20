# More Varaiables


## Reserved variables

### Command line arguments

There exists some reserved variables in sh, some of these are : 
- $0-$9 : These are the parameters the program is called with, $0 is the file name its currently 
  executing [Just like C]
- $@ : This gives all the parameters entered within a iterable data type [Example in main.sh]
- $\* : This also gives all the parameters that omits white spaces and quotes
- $? : This stores the exit code of the last run command in the system.
  0 is everyting is good,else 1. Yeah I know counter intiuitve [Like C].
general rule of thumb, use @ and avoid *

- $# : this gives the number of 

Does this mean we can not take more than 9 command line argument?HAHAHH No.

We make use of shift, meaning we actually have all the arguments passsed, simply the ones are not
accesssible cus we havent assigned them to variables.

when we shift, the $1 parameters is dropped than everything is moved down the line making 10th
variable accesssible. [Example in main.sh]

### More reserver variables

- $$ : This is the PID[Process IDentifier] of the current running script, this often used to create 
  files [when needed so] when many instances of the same thread is running.
- $! : This gives the PID of the last run process in the background.
 
Another very interseting variable is called IFS.
what is this IFS? It stands for Internal Field Seperator, this is more like being able to do 
formatted inputs. It's oten useful to have the default IFS sotred else where, Yes Shell script
by default has some values that indicated spilt b/w mutiple variable input.
The resouces claims this default value is : space tab newline, but i suspect it's way more.
When copy the IFS value onto new var just wrap the old ones around "" cus the default ones may have
some variable that can not be handled.

What does the default value of IFS do? well when two words are seperated by spaces, they are
considered to be two variables. [Examples in main.sh]
