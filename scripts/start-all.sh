#!/bin/sh

set -euo pipefail

# Export the configuration values
. /home/pi/hydrophonitor/scripts/export-config-values.sh

# Create output directory
OUTPUT_DIR=$BASE_DIR_PATH/$(date +"%Y-%m-%d_%H-%M-%S_output")

mkdir -p $OUTPUT_DIR

(export OUTPUT_DIR=$OUTPUT_DIR; trap 'kill 0' SIGINT; ./scripts/start-gps.sh & ./scripts/start-audio.sh & ./scripts/start-pressure-depth.sh)
