#!/bin/bash

set -x

echo "Setting up the real time clock module, part 1"

# Enable i2c
sudo raspi-config nonint do_i2c 0

# Enable i2c rtc on the default i2c pins
sudo cat << EOF | sudo tee -a /boot/config.txt
dtoverlay=i2c-rtc,ds3231
EOF

# Disable fake-hwclock
sudo apt-get remove -y fake-hwclock
sudo update-rc.d -f fake-hwclock remove
sudo systemctl disable fake-hwclock

# Load needed modules at boot
sudo cat << EOF | sudo tee -a /etc/modules
i2c-bcm2708
i2c-dev
rtc-ds1307
EOF

# Remove some lines from /lib/udev/hwclock-set
sudo sed -i '/^if \[ \-e \/run\/systemd\/system \] ; then$/,/^fi$/d' /lib/udev/hwclock-set

echo "Setup RTC part 1 done, reboot and run setup-rtc-2.sh"
