# Hydrophonitor

A software package to record audio and related metadata from a configuration
of hydrophones.

## Overview

Module           | Description
-----------------|-
audio-logger     | Receive an audio signal from the DAC and write it on disk in `.wav` format.
gps-logger       | Record position and time of the device in `.csv` format.
depth-logger     | Record depth of the device and save it in `.csv` format.
*lcd-display     | Provide information on the device's LCD screen
*device-controls | Provide device control using physical buttons.

## Data Formats

Type | Format
-----|-
GPS Data |