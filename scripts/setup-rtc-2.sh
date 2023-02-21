#!/bin/bash

set -x

echo "Setting up the real time clock module, part 2"

I2C_BUS=1

echo ds1307 0x68 | sudo tee /sys/class/i2c-adapter/i2c-$I2C_BUS/new_device

# Load RTC clock at boot
config="echo ds1307 0x68 | sudo tee /sys/class/i2c-adapter/i2c-$I2C_BUS/new_device
sudo hwclock -s
exit 0"

if ! grep -q "$config" /etc/rc.local; then
  sudo sed -i "s/^exit 0$//" /etc/rc.local
  echo "$config" | sudo tee -a /etc/rc.local
fi

# Set system time to Internet time
echo "Restarting systmd-timesyncd to update system time"
sudo systemctl restart systemd-timesyncd

echo "System time now:"
date

sleep 5

# Write system time to the RTC module
echo "Hardware clock time now:"
sudo hwclock -r --verbose

echo "Writing system time to hardware clock"
sudo hwclock -w --verbose

echo "Hardware clock time now:"
sudo hwclock -r --verbose
