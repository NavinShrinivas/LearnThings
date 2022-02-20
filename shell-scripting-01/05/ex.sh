#!/bin/sh


test="test"

while [ $test != "exit" ]
do
    echo "Enter intput [\"exit\" for quitting] : "
    read test
    if [ $test = "exit" ];then
        echo "Bye bye :)"
    else
        echo $test | grep -Eo "[^0-9]" > /dev/null
        if [ $? -eq "0" ];then
            echo "HMM, you have given a text that is not \"exit\"."
        else
            echo "purely a number"
        fi
    fi
done

