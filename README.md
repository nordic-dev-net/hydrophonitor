# Hydrophonitor

A software package to record audio and related metadata from a configuration
of hydrophones.

For setup instructions, see `docs/setup.md`.

## Overview

Module           | Description
-----------------|-
audio-logger     | Receive an audio signal from the DAC and write it on disk in `.wav` format.
gps-logger       | Record position and time of the device in `.csv` format.
depth-logger     | Record depth of the device and save it in `.csv` format.
*lcd-display     | Provide information on the device's LCD screen
*device-controls | Provide device control using physical buttons.

*) todo, not implemented yet

## Data Formats

Type        | Output file format | Output file name                     | Output structure | Content
------------|--------------------|--------------------------------------|------------------|-
Audio Data  | .wav               | <start-time-of-recording>_audio.wav  | Each recorded chunk will be written to its own file in `audio` folder | Wav audio data, configuration defined in XXX
GPS Data    | .csv               | <start-time-of-recording>_gps.wav    | All data written to a single file | Csv data with following fields: GPS time UTC, latitude, longitude, speed, satellites in view
Depth data  | .csv               | <start-time-of-recording>_depth.wav  | All data written to a single file | Csv data with following fields: date and time, voltage of depth sensor (V), depth (m)
Log data    | .txt               | <start-time-of-recording>_log.txt    | All data written to a single file | Text file where each entry contains the following: date and time, process that writes the entry, logged information

## Output Locations

The location for the output directories is defined by a configurable value OUTPUT_PATH in `hydrophonitor-config.txt`. If directories along this path do not exist, they will be created. If an error occurs or the location is not writable, output will be written to the default location (DEFAULT_OUTPUT_PATH in hydrophonitor-config.txt) instead.

SSD card mounting is not yet configured in the setup.

A recording session starts when the Raspberry Pi is turned on or booted, and ends on shutdown. Each session will have its output written in its own directory that will be named <start-time-of-recording>_recordings. 