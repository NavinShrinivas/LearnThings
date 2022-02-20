#!/bin/sh

echo -n "Enter your name for user [`whoami`] (Default value username): "
read name
echo "name entered : ${name:-`whoami`}"
echo "Enter age (Default value 18): "
read age
#${age:=18} #works but gives errors
$age=${age:-18} #can be done in line of echo as well
echo "age entered : $age"
