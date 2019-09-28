#!/bin/bash

if [ "$EUID" -ne 0 ]
    then echo "please run as root"
    exit
fi

cp target/release/twilight-commander /usr/local/bin/twilight-commander

echo "twilight-commander was installed/updated"
