#!/bin/sh

# Export the configuration values
. /home/pi/hydrophonitor/scripts/export-config-values.sh

# Create output directory
OUTPUT_DIR=$BASE_DIR_PATH/$(date +"%Y-%m-%d_%H-%M-%S_output")
mkdir -p $OUTPUT_DIR/audio

# Sleep for a little to wait for GPS and sound card to be ready
sleep 10

echo "(export OUTPUT_DIR=$OUTPUT_DIR; /home/pi/hydrophonitor/scripts/start-audio.sh & /home/pi/hydrophonitor/scripts/start-gps.sh)" >> $OUTPUT_DIR/log.txt 2>&1

(export OUTPUT_DIR=$OUTPUT_DIR; /home/pi/hydrophonitor/scripts/start-audio.sh & /home/pi/hydrophonitor/scripts/start-gps.sh) >> $OUTPUT_DIR/log.txt 2>&1
