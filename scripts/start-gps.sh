#!/usr/bin/sh

# Export the configuration values
. /home/pi/hydrophonitor/scripts/export-config-values.sh

GPS_EXECUTABLE_LOCATION="/home/pi/hydrophonitor/gps-logger"
OPTIONS="--output $OUTPUT_DIR --interval $GPS_INTERVAL"

cd $GPS_EXECUTABLE_LOCATION && python record-gps.py $OPTIONS
