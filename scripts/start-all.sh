#!/bin/bash

# Print all commands to standard output
set -x

SCRIPT_PATH=/home/pi/hydrophonitor/scripts

# Export the configuration values
. $SCRIPT_PATH/export-config-values.sh

# Create output directory
OUTPUT_DIR=$OUTPUT_PATH/$(date +"%Y-%m-%d_%H-%M-%S_output")
echo "Create output directory $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"/audio

# Sleep for a little to wait for GPS and sound card to be ready
sleep 10

(export OUTPUT_DIR=$OUTPUT_DIR; $SCRIPT_PATH/start-audio.sh & $SCRIPT_PATH/start-gps.sh & $SCRIPT_PATH/start-pressure-depth.sh) >> "$OUTPUT_DIR"/log.txt 2>&1
