#! /bin/bash

function read_dir(){
for file in `ls`
do
    if [ -d $file ] && [[ $file = course-* ]]
    then
        head -1 $file/README.md
    fi
done
}

read_dir $1
