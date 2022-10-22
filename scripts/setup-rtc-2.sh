#!/bin/bash

set -x

echo "Setting up the real time clock module, part 2"

echo ds1307 0x68 | sudo tee /sys/class/i2c-adapter/i2c-3/new_device

# Load RTC clock at boot
sudo sed -i "s/^exit 0$//" /etc/rc.local
sudo cat << EOF | sudo tee -a /etc/rc.local
echo ds1307 0x68 | sudo tee /sys/class/i2c-adapter/i2c-3/new_device
sudo hwclock -s
exit 0
EOF

# Set system time to Internet time

echo "Restarting systmd-timesyncd to update system time"

sudo systemctl restart systemd-timesyncd

echo "System time now:"
date

# Write system time to the RTC module

echo "Hardware clock time now:"
sudo hwclock -r

echo "Writing system time to hardware clock"
sudo hwclock -w

echo "Hardware clock time now:"
sudo hwclock -r
