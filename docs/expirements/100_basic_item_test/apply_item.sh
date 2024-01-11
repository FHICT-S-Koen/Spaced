#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <number_of_iterations>"
  exit 1
fi

number_of_iterations=$1

for ((i = 1; i <= number_of_iterations; i++)); do
  echo "Applying SQL statement for iteration $i"
  x=$((RANDOM % 100))
  y=$((RANDOM % -100))
  psql postgres://admin:password@localhost:5432/spaced -f $( dirname "${BASH_SOURCE[0]}" )/basic_item.sql -v x="$x" -v y="$y"
done

echo "Script completed"
