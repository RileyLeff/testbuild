#!/bin/bash

# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 pattern env_var_name"
    exit 1
fi

pattern=$1
env_var_name=$2

# Get the first file that matches the pattern
file=$(ls $pattern | head -n 1)

# If a file was found
if [ -n "$file" ]; then
    # Extract the extension
    extension=$(basename "$file" | rev | cut -d. -f1 | rev)
    # Save the extension to an environment variable
    export $env_var_name=".$extension"
fi