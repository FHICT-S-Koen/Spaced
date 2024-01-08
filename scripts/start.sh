#!/bin/bash

# Parse command-line arguments
while [[ "$#" -gt 0 ]]; do
  case $1 in
    -p|--producer)
      producer="$2"
      shift
      ;;
    *)
      # echo "Unknown parameter: $1"
      # exit 1
      ;;
  esac
  shift
done

# item_producer="${!p}"

cleanup() {
  echo "Cleaning up..."
  pkill -P $$
}

trap 'cleanup' EXIT

echo "${!producer}"

echo "Running services..."
# cargo run -p item_consumer &
  for value in "${!p[@]}"; do
    cargo run -p item_producer -- --port $value &
  done
wait
