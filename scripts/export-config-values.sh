#!/bin/bash

if [ $# -eq 1 ]; then
	CONFIG_FILE=$1
else
	CONFIG_FILE=/hydrophonitor/hydrophonitor-config.txt
fi

# Select non-comment lines in CONFIG_FILE, clean horizontal whitespace
args=$(grep -v '^#' $CONFIG_FILE | tr -d '[:blank:]' | tr '\n' ' ')

for arg in $args; do
	echo "export $arg"
	export "${arg?}"
done
