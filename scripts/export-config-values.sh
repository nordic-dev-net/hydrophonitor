#!/bin/bash

CONFIG_FILE=/boot/hydrophonitor/configuration/hydrophonitor-config.txt

export $(grep -v '^#' $CONFIG_FILE | tr -d '[:space:]' | xargs -d '\n')
