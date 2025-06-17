#!/bin/bash

month=$(date +%b | tr '[:upper:]' '[:lower:]') # Month in lowercase
day=$(date +%d) # Day of the month
mkdir -p "logs/$month/$day"
cargo bench > "logs/$month/$day/bench-$(date +%H-%M-%S).log"
