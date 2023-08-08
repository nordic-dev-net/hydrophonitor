#!/bin/bash

set -x

# Export the configuration values
SCRIPT_PATH=/hydrophonitor/scripts
. $SCRIPT_PATH/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $GPS_INTERVAL_SECS"

cd "$INSTALL_PATH"/gps-logger && python record-gps.py $OPTIONS
