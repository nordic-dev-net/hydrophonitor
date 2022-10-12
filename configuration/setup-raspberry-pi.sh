#!/bin/bash

set -ex

DIR_PATH=$HOME
BOOT_DIR_PATH=/boot/hydrophonitor

# Copy the files to DIR_PATH
echo
echo "### Copy files to $DIR_PATH"
echo

mkdir -p "$DIR_PATH"
cd "$DIR_PATH"
cp -R $BOOT_DIR_PATH/ .

# Install the Rust toolchain
echo
echo "### Install the Rust toolchain"
echo

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Install some developer tools
echo
echo "### Install some developer tools"
echo

sudo apt-get update && sudo apt-get install -y build-essential

# Setup audio
echo
echo "### Setup audio"
echo

cd "$DIR_PATH" && sh hydrophonitor/scripts/setup-audio.sh

# Setup GPS
echo
echo "### Setup GPS"
echo

cd "$DIR_PATH" && sh hydrophonitor/scripts/setup-gps.sh

# Setup depth sensor
echo
echo "### Setup depth recording"
echo

cd "$DIR_PATH" && sh hydrophonitor/scripts/setup-pressure-depth.sh

# Set up cron job to start the recordings at boot
echo
echo "### Set up a cron job to start the recordings at boot"
echo

# USER=$(whoami)
CRON_FILE=/etc/crontab
CRON_COMMAND="@reboot root $DIR_PATH/hydrophonitor/scripts/start-all.sh 2>&1 >> $BOOT_DIR_PATH/log.txt"

# Append command to cron file only if it's not there yet
sudo grep -qxF "$CRON_COMMAND" $CRON_FILE || echo "$CRON_COMMAND" | sudo tee -a $CRON_FILE

# Reboot
echo
echo "### Setup ready, run 'sudo reboot' to apply all changes"
echo
