#!/bin/sh

set -euo pipefail

# Copy the files to /home/pi
cd /home/pi
cp -R /boot/hydrophonitor .
cd hydrophonitor

# Install the Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Setup audio
sh scripts/setup-audio.sh

# Setup GPS
sh scripts/setup-gps.sh

# Setup depth sensor
sh scripts/setup-pressure-depth.sh

# Setup cron job to start the recordings at boot
CRON_FILE=/etc/crontab
echo "@reboot root /home/pi/hydrophonitor/scripts/start-all.sh" >> $CRON_FILE

# Reboot
sudo reboot
