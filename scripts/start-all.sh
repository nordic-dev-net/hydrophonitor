#!/bin/sh

(trap 'kill 0' SIGINT; ./scripts/start-gps.sh & ./scripts/start-audio.sh)
