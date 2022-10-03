#!/usr/bin/sh

# Export the configuration values
. /home/pi/hydrophonitor/scripts/export-config-values.sh

DEPTH_EXECUTABLE_LOCATION="/home/pi/hydrophonitor/depth-logger"

OPTIONS="--output $OUTPUT_DIR --interval $DEPTH_INTERVAL"

cd $DEPTH_TARGET_LOCATION && python record-depth.py $OPTIONS
