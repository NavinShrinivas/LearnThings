#!/bin/sh


echo "gimeee some input : "
read var

if [ $var != "bye" ]
then
    echo "not the same my boy,"
else
    echo "yeh they same"
fi

var2="hello1"

while [ $var2 != "hello" ];do
if [ $var2 != "hello" ];then
    echo "I wont finish this until you greet me properly [INPUT] : "
    read var2
elif \
    [ $var2 = "hi" ]
then
    echo "Almost there,but not yet [INPUT] : "
    read var2
fi
done

vara=1
varb="1"
if [ $vara = $varb ];then
    echo "ooooh,js crap"
fi

if [ -n $vara ];then
    echo "yess, none zero"
else
    echo "Ahhh,issa zero"
fi

if [ -n $varb ];then
    echo "yess, none zero"
else
    echo "Ahhh,issa zero"
fi

