#!/bin/bash

kill_ll() {
  if [[ $(is_ll) == "true" ]]; then
    pkill -f "Lavalink.jar"
    echo "Killed Lavalink"
  else
    echo "Lavalink isn't running"
  fi
}

start_ll() {
  if [[ $(is_ll) == "true" ]]; then
    echo "Lavalink already running"
  else
    echo "Starting Lavalink"
    { java -jar Lavalink.jar >/dev/null & } 2>&1
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

is_ll() {
  pid=$(pgrep -f "Lavalink.jar")

  if [[ -n "$pid" ]]; then
    echo true
  else
    echo false
  fi
}

update_ll() {
  latest=$(curl --silent "https://api.github.com/repos/freyacodes/Lavalink/releases/latest" | jq -r .tag_name)

  if [ -f "Lavalink.jar" ]; then
    echo "Downloading Lavalink"
    curl -fsLO https://github.com/freyacodes/Lavalink/releases/download/"$latest"/Lavalink.jar
    start_ll
  else
    current=$(java -jar Lavalink.jar --version | grep "Version" | grep -Eo "[0-9]+\.[0-9]+")

    if [ "$latest" != "$current" ]; then
      kill_ll

      echo "Removing old Lavalink"
      rm Lavalink.jar

      echo "Downloading new Lavalink"
      curl -fsLO https://github.com/freyacodes/Lavalink/releases/download/"$latest"/Lavalink.jar

      start_ll
    else
      echo "Lavalink is already up to date"
    fi
  fi
}

help() {
  printf "\nUsage: Lavalink [-ikrush]\n\nOptions:\n  -i    State whether Lavalink is running\n  -k    Kill all Lavalink processes\n  -r    Restart Lavalink\n  -u    Update Lavalink\n  -s    Start Lavalink\n  -h    Display this message\n"
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
