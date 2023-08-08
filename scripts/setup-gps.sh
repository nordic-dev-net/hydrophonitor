#!/bin/bash

set -ex

echo "Setting up GPS recording"

sudo apt-get update && sudo apt-get install -y gpsd gpsd-clients

sudo pip install -r gps-logger/requirements.txt

sudo systemctl stop gpsd.socket
sudo systemctl disable gpsd.socket

device="/dev/ttyUSB0"

sudo gpsd ${device} -F /var/run/gpsd.sock

sudo sed -i "s|DEVICES=\"\"|DEVICES=\"${device}\"|g" /etc/default/gpsd

config="START_DAEMON=\"true\""

if ! grep -q "$config" /etc/default/gpsd; then
  echo "$config" | sudo tee -a /etc/default/gpsd
fi
