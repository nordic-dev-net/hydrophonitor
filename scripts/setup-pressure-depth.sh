#!/bin/sh

set -ex

echo "Setting up depth recording"

# Enable i2c
sudo raspi-config nonint do_i2c 0

# Enable i2c on bus 3 and GPIO pins sda=23 and scl=24
sudo cat << EOF | sudo tee -a /boot/config.txt
dtoverlay=i2c-gpio,bus=3,i2c_gpio_sda=23,i2c_gpio_scl=24
EOF

# Install packages
sudo apt-get update && sudo apt-get install -y i2c-tools python3-pip

sudo pip install -r $HOME/hydrophonitor/depth-logger/requirements.txt
