#!/bin/sh

set -ex

echo "Setting up depth recording"

# Enable i2c bus on Raspberry Pi
sudo raspi-config nonint do_i2c 0

# Install packages
sudo apt-get update && sudo apt-get install -y i2c-tools python3-pip

sudo pip install -r $HOME/hydrophonitor/depth-logger/requirements.txt
