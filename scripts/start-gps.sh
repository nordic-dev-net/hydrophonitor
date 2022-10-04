#!/usr/bin/sh

# Export the configuration values
. /home/pi/hydrophonitor/scripts/export-config-values.sh

GPS_EXECUTABLE_PATH="/home/pi/hydrophonitor/gps-logger"

OPTIONS="--output $OUTPUT_DIR --interval $GPS_INTERVAL"

echo "cd $GPS_EXECUTABLE_PATH && python record-gps.py $OPTIONS"

cd $GPS_EXECUTABLE_PATH && python record-gps.py $OPTIONS
