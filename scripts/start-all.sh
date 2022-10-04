#!/bin/sh

# Export the configuration values
. /home/pi/hydrophonitor/scripts/export-config-values.sh

# Create output directory
OUTPUT_DIR=$BASE_DIR_PATH/$(date +"%Y-%m-%d_%H-%M-%S_output")

mkdir -p $OUTPUT_DIR/audio

sleep 10

# (export OUTPUT_DIR=$OUTPUT_DIR; trap 'kill 0' SIGINT; /home/pi/hydrophonitor/scripts/scripts/start-gps.sh & /home/pi/hydrophonitor/scripts/start-audio.sh & /home/pi/hydrophonitor/scripts/start-pressure-depth.sh)

echo "(export OUTPUT_DIR=$OUTPUT_DIR; /home/pi/hydrophonitor/scripts/start-audio.sh & /home/pi/hydrophonitor/scripts/start-gps.sh)" >> $OUTPUT_DIR/log.txt 2>&1

(export OUTPUT_DIR=$OUTPUT_DIR; /home/pi/hydrophonitor/scripts/start-audio.sh & /home/pi/hydrophonitor/scripts/start-gps.sh) >> $OUTPUT_DIR/log.txt 2>&1
