#!/bin/bash

CONFIG_FILE=$1

export $(grep -v '^#' $CONFIG_FILE | tr -d '[:space:]' | xargs -d '\n')
