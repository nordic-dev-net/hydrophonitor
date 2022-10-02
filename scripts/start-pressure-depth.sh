#!/usr/bin/sh

DEPTH_TARGET_LOCATION="/home/shared/hydrophonitor/depth-logger"
OPTIONS=""

cd $DEPTH_TARGET_LOCATION && python record-depth.py $OPTIONS
