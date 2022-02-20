# Variables, expr and Input

Note : Please, this is important. Never put unessacary spaces anywhere.Shell scripts
Hate that.

## Variables

Variables assignment is quite staright forward, acessing these variables have to be done 
with '$' directive

### Scope of a variable in a shell

In Bourne shell if you try and access a variable that was'nt declared, It will give no 
warnings, it'll simply return a empty string.

When ever a shell script is excuted, a new shell is spawned. This is better understood
by the example below : 

```
❰navin❙~/github/PWM/shell-scripting-01/02(git✱main)❱✔≻ cat main.sh 
#!/bin/sh

first_var="hello world"
echo $first_var

echo what is you\'r damn name?
read his_name
echo Yoiu\'r name is $his_name

echo "Set var was $VAR"
VAR="hewwo UwU"
echo "Set var was $VAR"
❰navin❙~/github/PWM/shell-scripting-01/02(git✱main)❱✔≻ set VAR "helloworld"
❰navin❙~/github/PWM/shell-scripting-01/02(git✱main)❱✔≻ ./main.sh 
hello world
what is you'r damn name?
navin
Yoiu'r name is navin
Set var was 
Set var was hewwo UwU
```
The above makes it quite clearn on the "new" shell being spawned. How do you get around
this? Well we need to export any variables we set to be used by other shells and programs

Like : 
```
❰navin❙~/github/PWM/shell-scripting-01/02(git✱main)❱✔≻ export set VAR "helloworld"
❰navin❙~/github/PWM/shell-scripting-01/02(git✱main)❱✔≻ ./main.sh
hello world
what is you'r damn name?
navin
Yoiu'r name is navin
Set var was helloworld
Set var was hewwo UwU
```
If you observe right, we are also making changes to this varible, but once the shell
script is done this new value is lost. How do we save this in the "export" space?
This can be done quite easily! simply change the way the shell scipt is excuted,like :
```
[Prompt]$ . ./main.sh
```
>Note : This way of executing shell scripts does not work in all shells,like the shell i am using "fish" doesn't like it.
>on further thinking, this is cus of fish not knowing what to do with "=" and you may have very well seen the set clause used earlier. Anyhow, I'll not be getting into it any further and use bash shell.

this leads to : 
```
❰navin❙~/github/PWM/shell-scripting-01/02(git≠✱main)❱✘≻ bash
[navin@navin-omenlaptop15en0xxx 02]$ . ./main.sh 
hello world
what is you'r damn name?
navin
You'r name is navin
Set var was helloworld
Set var was hewwo UwU
[navin@navin-omenlaptop15en0xxx 02]$ echo $VAR 
hewwo UwU
[navin@navin-omenlaptop15en0xxx 02]$
```
## expr

Expr behaves quite strangley, which can be seen in "1.png" in this folder, this we will
see later down the process.

As of now all you need to know, expr doesnt show diff b/w varibles [numbers] that are 
enclosed in "" or simply given as numbers.

## Inputs

keyword used "read", more specifically, read VARIABLE_NAME. This varible can later be
accessed using $ else where

## Excersise

Create a program to read a file name from user, make a file with the given name appended
with "_file", good luck!

Yeah 02 was definetly heavy, fret not. Take a break if needed...Come back and let's move
to 03.



