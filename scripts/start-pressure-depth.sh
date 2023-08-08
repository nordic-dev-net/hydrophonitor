#!/bin/bash

set -x

I2C_BUS=3

# Export the configuration values
SCRIPT_PATH=/hydrophonitor/scripts
. $SCRIPT_PATH/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $DEPTH_INTERVAL_SECS --bus $I2C_BUS"

cd "$INSTALL_PATH"/depth-logger && python record-depth.py $OPTIONS
