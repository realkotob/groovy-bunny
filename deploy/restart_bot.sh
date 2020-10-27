#!/bin/bash


#cd "$( dirname "${BASH_SOURCE[0]}" )"
cd /root/chrono-rabbit/

git pull --rebase; 
cargo run --release &
