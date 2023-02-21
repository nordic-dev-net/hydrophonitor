#!/bin/bash

target/release/audio rec \
	--name test \
	--output recordings/ \
	--batch-recording 3 \
	--sample-rate 44100 \
	--channels 2 \
	--buffer-size 1024 \
	alsa \
