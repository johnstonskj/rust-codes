#/usr/bin/env sh

DATA_DIR=./data

if [[ ! -d $DATA_DIR ]] ; then 
    mkdir $DATA_DIR
fi

curl -o data/list-one.xml "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml"
curl -o data/list-three.xml "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-three.xml"

