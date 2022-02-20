echo Entered paramters are :

for i in $@
do
    echo "parameter : $i"
done

## For more than 9 variables
# previous also work for more than 9 variables, just saying.They simply don't have a reserved 
# variable
while [ $# != "0" ]
do
    echo "argument : $1"
    shift
done

oldIFS="$IFS"
IFS=? #Input splits using ?

echo -n "Enter a whole bunch of inputs seperated by ? : "
read x y z
echo "X is $x" 
echo "Y is $y"
echo "Z is $z"
  


