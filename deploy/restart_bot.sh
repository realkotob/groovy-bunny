#!/bin/bash

#cd "$( dirname "${BASH_SOURCE[0]}" )"
cd /root/chrono-rabbit/ ;

git pull --rebase ; 

pkill chrono_rabbit ;

cargo run --release &
