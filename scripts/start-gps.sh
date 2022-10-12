#!/bin/bash

# Export the configuration values
/home/pi/hydrophonitor/scripts/export-config-values.sh

OPTIONS="--output $OUTPUT_DIR --interval $GPS_INTERVAL"

cd /home/pi/hydrophonitor/gps-logger && python record-gps.py $OPTIONS
