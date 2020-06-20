#!/bin/bash

BOT_NAME=chrono_rabbit

git pull origin; 
cargo build --release; 
sudo pkill -kill $BOT_NAME;
./target/release/$BOT_NAME & &> /dev/null; 