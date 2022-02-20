#!/bin/sh

for i in 1 3 5 4 2
do
    echo "Value of i : $i"
done

for i in "value 1" "value 2" 3 * "end"
do
    echo "iterator values : $i"
done

str="hello"
while [ "$str" != "bye" ]
do
    echo "enter something else [bye for exit] : "
    read str
done

#while :
#do
    #echo "forevaaa and evvvaaaa"
#done

while read i
do
    case $i in
        "hello") echo "English";;
        "howdy") echo "American";;
        "bonjour") echo "French";;
        *) echo "Something else : $i";;
    esac
done < input.txt

