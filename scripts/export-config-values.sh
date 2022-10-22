#!/bin/bash

if [ $# -eq 1 ]; then
	CONFIG_FILE=$1
else
	CONFIG_FILE=/boot/hydrophonitor/hydrophonitor-config.txt
fi

export $(grep -v '^#' $CONFIG_FILE | tr -d '[:space:]' | xargs -d '\n')
