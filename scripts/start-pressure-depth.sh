#!/bin/bash

set -x

I2C_BUS=3

# Export the configuration values
SCRIPT_PATH=/home/pi/hydrophonitor/scripts
. $SCRIPT_PATH/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $DEPTH_INTERVAL --bus $I2C_BUS"

cd $HOME_PATH/hydrophonitor/depth-logger && python record-depth.py $OPTIONS
