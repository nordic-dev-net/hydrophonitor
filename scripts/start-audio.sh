#!/bin/sh

# Start jack server

sh scripts/start-jack.sh

# Start recording

cd audio-logger && cargo run