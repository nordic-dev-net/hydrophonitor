#!/bin/bash

set -x

# Export the configuration values
SCRIPT_PATH=/home/pi/hydrophonitor/scripts
. $SCRIPT_PATH/export-config-values.sh

AUDIO_TARGET_EXECUTABLE="audio"

OPTIONS="rec \
--name audio_data \
--output $OUTPUT_DIR/audio \
--batch-recording $BATCH_RECORD_LENGTH \
--sample-rate $SAMPLE_RATE \
--channels $CHANNELS \
--buffer-size 1024 \
alsa"

cd $HOME_PATH/hydrophonitor/audio-logger/target/release && ./$AUDIO_TARGET_EXECUTABLE $OPTIONS
