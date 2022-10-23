#!/bin/bash

set -x

# Export the configuration values
SCRIPT_PATH=/home/pi/hydrophonitor/scripts
. $SCRIPT_PATH/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $DEPTH_INTERVAL"

cd $HOME_PATH/hydrophonitor/depth-logger && python record-depth.py $OPTIONS
