#!/usr/bin/sh

GPS_TARGET_LOCATION="/home/shared/hydrophonitor/gps-logger"
OPTIONS=""

cd $GPS_TARGET_LOCATION && python record-gps.py $OPTIONS
