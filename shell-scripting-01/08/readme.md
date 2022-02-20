# More about Variables

In first variable module we use {} for fancy variable/text printing and handling. This has one more
use, It can be used to handle null or undefined variables[They aren't diff in shell].
Using those {} we can give the variable a default value [Example in main.sh].

- ${var :- value} : prints the value if var is not set
- ${var := value} : set the value of var is not set [not sure why, this works butt giving me error]
