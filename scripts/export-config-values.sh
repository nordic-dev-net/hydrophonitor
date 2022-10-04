#!/bin/sh

CONFIG_FILE=/boot/hydrophonitor/configuration/hydrophonitor-config.txt

export $(grep -v '^#' $CONFIG_FILE | xargs -d '\n')
