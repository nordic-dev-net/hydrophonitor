#!/bin/sh

set -ex

echo "Setting up GPS recording"

sudo apt-get update && sudo apt-get install -y gpsd gpsd-clients

sudo pip install -r $HOME/hydrophonitor/gps-logger/requirements.txt

sudo systemctl stop gpsd.socket
sudo systemctl disable gpsd.socket

device="/dev/ttyUSB0"

sudo gpsd ${device} -F /var/run/gpsd.sock

sudo sed -i "s|DEVICES=\"\"|DEVICES=\"${device}\"|g" /etc/default/gpsd

sudo grep -qxF "START_DAEMON=\"true\"" /etc/default/gpsd || echo "START_DAEMON=\"true\"" | sudo tee -a /etc/default/gpsd
