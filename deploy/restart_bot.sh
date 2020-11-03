#!/bin/bash

pkill chrono_rabbit ;
#cd "$( dirname "${BASH_SOURCE[0]}" )"
cd /root/chrono-rabbit/ ;

git pull --rebase ; 
cargo run --release &
