#!/bin/sh

CONFIG_FILE=/boot/hydrophonitor/hydrophonitor-config.txt

export $(grep -v '^#' $CONFIG_FILE | xargs -d '\n')
