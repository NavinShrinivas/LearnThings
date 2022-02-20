# Wildcards, Escape chars


## Wildcards
If you have used any unix system for a decent amount of time. wildcards should be noting
new to you.

Say u want to copy milliom files from
1./tmp/a/ to /tmp/b/
2.Copy all html file from b/w the make path
3.all files that start start with abc
answer for the following would be : 
```
$ cp /tmp/a/* /tmp/b/
$ cp /tmp/a/*.html /tmp/b/
$ cp /tmp/a/abc* /tmp/b/
```

## Escape Chars

If you have learnt or worked with any programmin language, you alreadry know this as well.
and I've aldready used escape chars in my previous examples.

most charecters such as \*,',... are interpreted literally when inside "diuble quoutes",
but few charecters such as $,\`,$ are not, to consider these literally we need to prepend
them  with a back slask "\\".

This in turn leads "\\" to also be special and needs to be escaped to be considered 
literally.

03 was really light, we have big things coming up ahead, LOOPS! Don't stop, keep moving!

Ah one more thing to add, what does \`\` do when not escaped in a "", it executes whatever is 
withing it.
