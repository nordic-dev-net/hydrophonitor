#!/bin/bash

set -ex

SDA_GPIO_PIN=10
SCL_GPIO_PIN=11
I2C_BUS=3

echo "Setting up depth recording"

# Enable i2c
sudo raspi-config nonint do_i2c 0

# Enable i2c on bus 3 and GPIO pins sda=23 and scl=24
config="dtoverlay=i2c-gpio,bus=$I2C_BUS,i2c_gpio_sda=$SDA_GPIO_PIN,i2c_gpio_scl=$SCL_GPIO_PIN"

if ! grep -q "$config" /boot/config.txt; then
  echo "$config" | sudo tee -a /boot/config.txt
fi

# Install packages
sudo apt-get update && sudo apt-get install -y i2c-tools python3-pip

sudo pip install -r $HOME/hydrophonitor/depth-logger/requirements.txt
