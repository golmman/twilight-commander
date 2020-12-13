#!/bin/bash

if [ "$EUID" -ne 0 ]
    then echo "please run as root"
    exit
fi

cp target/release/twilight-commander /usr/local/bin/twilight-commander
cp tcide_vim /usr/local/bin/tcide_vim
cp tcide_neovim /usr/local/bin/tcide_neovim
cp tcide /usr/local/bin/tcide

echo "twilight-commander was installed/updated"
