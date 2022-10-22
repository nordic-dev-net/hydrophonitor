#!/bin/bash

set -x

echo "Setting up the real time clock module, part 1"

# Enable i2c rtc on bus 3, set GPIO pins sda=23 and sdl=24
sudo cat << EOF | sudo tee -a /boot/config.txt
dtoverlay=i2c-gpio,bus=3,i2c_gpio_delay_us=1,i2c_gpio_sda=23,i2c_gpio_scl=24
dtoverlay=i2c-rtc,ds3231
EOF

# Disable fake-hwclock
sudo apt-get remove fake-hwclock
sudo update-rc.d -f fake-hwclock remove
sudo systemctl disable fake-hwclock

# Load modules at boot
sudo cat << EOF | sudo tee -a /etc/modules
i2c-bcm2708
i2c-dev
rtc-ds1307
EOF

# Remove some lines from /lib/udev/hwclock-set
sudo sed -i '/^if \[ \-e \/run\/systemd\/system \] ; then$/,/^fi$/d' /lib/udev/hwclock-set

echo "Setup RTC part 1 done, reboot and run setup-rtc-2.sh"