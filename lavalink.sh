#!/bin/bash

is_ll() {
  pid=$(pgrep -f "Lavalink.jar")

  if [[ -n "$pid" ]]; then
    echo true
  else
    echo false
  fi
}

kill_ll() {
  if [[ $(is_ll) == "true" ]]; then
    pkill -f "Lavalink.jar"
    echo "Killed Lavalink"
  else
    echo "Lavalink isn't running"
  fi
}

restart_ll() {
  kill_ll

  while [[ $(is_ll) == "true" ]]; do
    sleep 1
    kill_ll
  done

  sleep 5
  start_ll
}

start_ll() {
  if [[ $(is_ll) == "true" ]]; then
    echo "Lavalink already running"
  else
    echo "Starting Lavalink..."
    nohup java -jar Lavalink.jar & disown
  fi
}

update_ll() {
  latest=$(curl --silent "https://api.github.com/repos/freyacodes/Lavalink/releases/latest" | jq -r .tag_name)

  if [ -f "Lavalink.jar" ]; then
    current=$(java -jar Lavalink.jar --version | grep "Version" | grep -Eo "[0-9]+\.[0-9]+")

    if [ "$latest" != "$current" ]; then
      kill_ll

      echo "Removing old Lavalink"
      rm Lavalink.jar

      echo "Downloading new Lavalink"
      curl -fsLO https://github.com/freyacodes/Lavalink/releases/download/"$latest"/Lavalink.jar
    else
      echo "Lavalink is already up to date"
    fi
  else
    echo "No Lavalink jar found, downloading Lavalink..."
    curl -fsLO https://github.com/freyacodes/Lavalink/releases/download/"$latest"/Lavalink.jar
  fi

  start_ll
}

help() {
  printf "
  Usage: lavalink.sh [-ikrush]

  Options:
    -i    States whether Lavalink is running
    -k    Kills all Lavalink processes
    -r    Restarts Lavalink
    -u    Updates and starts Lavalink
    -s    Starts Lavalink
    -h    Displays this message
  "
}

while getopts ikrush flag; do
  case "${flag}" in
    i) is_ll ;;
    k) kill_ll ;;
    r) restart_ll ;;
    u) update_ll ;;
    s) start_ll ;;
    h) help ;;
    *) help ;;
  esac
done
