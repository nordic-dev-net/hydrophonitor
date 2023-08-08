#!/bin/bash

set -ex

BOOT_DIR_PATH=/boot
LOG_FILE=$BOOT_DIR_PATH/$(date +"%Y-%m-%dT%H-%M-%S")-setup-log.txt

# Output from commands within the curly braces is written
# to $LOG_FILE
{
echo
echo "Starting setup, writing logs to $LOG_FILE"
echo

echo
echo "### Read variables from config file"
echo

# Export variables from config file
CONFIG_FILE_PATH=$(dirname "$0")/../hydrophonitor-config.txt
. $(dirname "$0")/export-config-values.sh "$CONFIG_FILE_PATH"

echo "# Variables read from config file:"
echo "SAMPLE_RATE=$SAMPLE_RATE"
echo "CHANNELS=$CHANNELS"
echo "BITRATE=$BITRATE"
echo "BATCH_RECORD_LENGTH_SECS=$BATCH_RECORD_LENGTH_SECS"
echo "GPS_INTERVAL_SECS=$GPS_INTERVAL_SECS"
echo "DEPTH_INTERVAL_SECS=$DEPTH_INTERVAL_SECS"
echo "INSTALL_PATH=$INSTALL_PATH"
echo "OUTPUT_PATH=$OUTPUT_PATH"
echo "USER=$USER"
echo "PASSWORD=$PASSWORD"

echo
echo "### Create user"
echo

echo "$USER:$(openssl passwd -1 $PASSWORD)" | sudo tee /boot/userconf.txt

echo
echo "### Enable SSH"
echo

echo "" | sudo tee /boot/ssh.txt

echo
echo "### Create install directory"
echo

sudo mkdir -p "$INSTALL_PATH"

# Copy files to install directory if we are not already in it
if [ "$PWD" != "$INSTALL_PATH" ]; then
  echo
  echo "### Copy files to install directory"
  echo
  sudo cp -r . "$INSTALL_PATH"
fi

sudo chown -R $USER:$USER "$INSTALL_PATH"

# Install some development tools
echo
echo "### Install some development dependencies"
echo

sudo apt-get update && sudo apt-get install -y build-essential python3-pip

# Setup audio
echo
echo "### Setup audio"
echo

cd "$INSTALL_PATH" && ./scripts/setup-audio.sh

# Setup GPS
echo
echo "### Setup GPS"
echo

cd "$INSTALL_PATH" && ./scripts/setup-gps.sh

# Setup depth sensor
echo
echo "### Setup depth recording"
echo

cd "$INSTALL_PATH" && ./scripts/setup-pressure-depth.sh

# Setup shutdown button
echo
echo "### Setup shutdown button"
echo

cd "$INSTALL_PATH" && ./scripts/setup-shutdown-button.sh

# Set up cron job to start the recordings at boot
echo
echo "### Set up a cron job to start the recordings at boot"
echo

CRON_FILE=/etc/crontab
CRON_LOG_FILE="$INSTALL_PATH/cron-log.txt"
CRON_COMMAND="@reboot root $INSTALL_PATH/scripts/start-all.sh 2>&1 >> $CRON_LOG_FILE"

# Append command to cron file only if it's not there yet
if ! grep -q "$CRON_COMMAND" "$CRON_FILE"; then
  echo "$CRON_COMMAND" | sudo tee -a "$CRON_FILE"
fi

# Reboot
echo
echo "### Setup ready, run 'sudo reboot' to apply all changes"
echo

# Instructions for setting up the RTC module
echo
echo "### To set up the RTC module, run the following command:"
echo "cd ${INSTALL_PATH} && ./scripts/setup-rtc-1.sh"
echo "### Then, reboot the device:"
echo "sudo reboot"
echo "### Then, after reconnecting to the RPi, run the following command:"
echo "cd ${INSTALL_PATH} && ./scripts/setup-rtc-2.sh"
echo
} 2>&1 | sudo tee $LOG_FILE