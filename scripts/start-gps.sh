#!/bin/bash

set -x

# Export the configuration values
SCRIPT_PATH=/home/pi/hydrophonitor/scripts
$SCRIPT_PATH/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $GPS_INTERVAL"

cd $HOME_PATH/hydrophonitor/gps-logger && python record-gps.py $OPTIONS
