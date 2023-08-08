#!/bin/bash

set -x

# Export the configuration values
SCRIPT_PATH=/hydrophonitor/scripts
. $SCRIPT_PATH/export-config-values.sh

AUDIO_TARGET_EXECUTABLE="audio"

OPTIONS="rec \
--name audio_data \
--output $OUTPUT_DIR/audio \
--batch-recording $BATCH_RECORD_LENGTH_SECS \
--sample-rate $SAMPLE_RATE \
--channels $CHANNELS \
--buffer-size 1024 \
alsa"

cd "$INSTALL_PATH" && ./$AUDIO_TARGET_EXECUTABLE $OPTIONS
