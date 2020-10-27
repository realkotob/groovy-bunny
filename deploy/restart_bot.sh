#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

git pull origin; 
cargo build --release; 
sudo pkill -kill $BOT_NAME;
./target/release/$BOT_NAME & &> /dev/null; 