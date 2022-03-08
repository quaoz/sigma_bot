#!/bin/bash

update_bot() {
  git pull
  start_bot
}

start_bot() {
  cargo run --release --bin sigma_bot &
}

help() {
  printf "\nUsage: Bot.sh [-ush]\n\nOptions:\n  -u    Updates the bot\n  -s    Starts the bot\n  -h    Displays this message\n"
}

while getopts ush flag
do
    case "${flag}" in
        u) update_bot;;
        s) start_bot;;
        h) help;;
        *) help;;
    esac
done
