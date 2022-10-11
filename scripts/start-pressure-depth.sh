#!/bin/bash

# Export the configuration values
/home/pi/hydrophonitor/scripts/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $DEPTH_INTERVAL"

cd /home/pi/hydrophonitor/depth-logger && python record-depth.py $OPTIONS
