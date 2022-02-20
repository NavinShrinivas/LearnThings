#/bin/sh

echo "Enter name of file to be created : "
read FILENAME
echo "File with name ${FILENAME}_file created"
touch ${FILENAME}_file
