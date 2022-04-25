#!/bin/bash

is_bot() {
  pid=$(pgrep -f "sigma_bot")

  if [[ -n "$pid" ]]; then
    echo true
  else
    echo false
  fi
}

kill_bot() {
  if [[ $(is_bot) == "true" ]]; then
    pkill -f "sigma_bot"
    echo "Killed the bot"
  else
    echo "The bot isn't running"
  fi
}

restart_bot() {
  kill_bot

  while [[ $(is_bot) == "true" ]]; do
    sleep 1
    kill_bot
  done

  sleep 5
  start_bot
}

update_bot() {
  git pull
  start_bot
}

start_bot() {
  nohup cargo run --quiet --release --bin sigma_bot & disown
}

help() {
  printf "
  Usage: bot.sh [-ush]

  Options:
    -i    States whether the bot is running
    -k    Kills the bot
    -r    Restarts the bot
    -u    Updates and starts the bot
    -s    Starts the bot
    -h    Displays this message
  "
}

while getopts ikrush flag; do
  case "${flag}" in
    i) is_bot ;;
    k) kill_bot ;;
    r) restart_bot ;;
    u) update_bot ;;
    s) start_bot ;;
    h) help ;;
    *) help ;;
  esac
done
