#!/bin/bash

set -ex

echo "Setting up shutdown button"

config="dtoverlay=gpio-shutdown,gpio_pin=21,gpio_pull=up,active_low=1"

if ! grep -q "$config" /boot/config.txt; then
  echo "$config" | sudo tee -a /boot/config.txt
fi
