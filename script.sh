#!/bin/bash

# Check if primes.exe exists
if [ ! -f "./target/release/primes.exe" ]; then
    echo "Error: primes.exe not found"
    exit 1
fi

# Check if nums.txt exists
if [ ! -f "nums.txt" ]; then
    echo "Error: nums.txt not found"
    exit 1
fi

# Loop through each line in nums.txt
while IFS= read -r line; do
    # Replace <TARGET> with the actual number and execute primes.exe
    command="./target/release/primes.exe --action miller-rabin --target $line"
    eval $command
done < "nums.txt"
