#!/bin/bash

set -ex

BOOT_DIR_PATH=/boot/hydrophonitor
LOG_FILE=$BOOT_DIR_PATH/$(date +"%Y-%m-%dT%H-%M-%S")-setup-log.txt

# Output from commands within the curly braces is written
# to $LOG_FILE
{
DIR_PATH=$HOME

echo
echo "Starting setup, writing logs to $LOG_FILE"
echo

echo
echo "### Update file paths in config and start script files"
echo

# Update hydrophonitor-config.txt HOME_PATH to the current user's home
sudo sed -i "s|^HOME_PATH=.*$|HOME_PATH=$HOME|" $BOOT_DIR_PATH/hydrophonitor-config.txt

# Update script path in start scripts to the current user's home
sudo sed -i "s|^SCRIPT_PATH=.*$|SCRIPT_PATH=$HOME/hydrophonitor/scripts|" $BOOT_DIR_PATH/scripts/start-all.sh
sudo sed -i "s|^SCRIPT_PATH=.*$|SCRIPT_PATH=$HOME/hydrophonitor/scripts|" $BOOT_DIR_PATH/scripts/start-audio.sh
sudo sed -i "s|^SCRIPT_PATH=.*$|SCRIPT_PATH=$HOME/hydrophonitor/scripts|" $BOOT_DIR_PATH/scripts/start-gps.sh
sudo sed -i "s|^SCRIPT_PATH=.*$|SCRIPT_PATH=$HOME/hydrophonitor/scripts|" $BOOT_DIR_PATH/scripts/start-pressure-depth.sh

# Copy the files to DIR_PATH
echo
echo "### Copy files to $DIR_PATH"
echo

mkdir -p "$DIR_PATH"
cd "$DIR_PATH"
rm -rf hydrophonitor
cp -R $BOOT_DIR_PATH/ .

# Install some development tools
echo
echo "### Install some development tools"
echo

sudo apt-get update && sudo apt-get install -y build-essential python3-pip

# Install the Rust programming language toolchain
echo
echo "### Install the Rust toolchain"
echo

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Setup audio
echo
echo "### Setup audio"
echo

cd "$DIR_PATH" && ./hydrophonitor/scripts/setup-audio.sh

# Setup GPS
echo
echo "### Setup GPS"
echo

cd "$DIR_PATH" && ./hydrophonitor/scripts/setup-gps.sh

# Setup depth sensor
echo
echo "### Setup depth recording"
echo

cd "$DIR_PATH" && ./hydrophonitor/scripts/setup-pressure-depth.sh

# Setup shutdown button
echo
echo "### Setup shutdown button"
echo

cd "$DIR_PATH" && ./hydrophonitor/scripts/setup-shutdown-button.sh

# Set up cron job to start the recordings at boot
echo
echo "### Set up a cron job to start the recordings at boot"
echo

CRON_FILE=/etc/crontab
CRON_LOG_FILE="$BOOT_DIR_PATH/\$(date +\"%Y-%m-%dT%H-%M-%S\")-cron-log.txt"
CRON_COMMAND="@reboot root $DIR_PATH/hydrophonitor/scripts/start-all.sh 2>&1 > $CRON_LOG_FILE"

# Append command to cron file only if it's not there yet
if ! grep -q "$CRON_COMMAND" "$CRON_FILE"; then
  echo "$CRON_COMMAND" | sudo tee -a "$CRON_FILE"
fi

# Reboot
echo
echo "### Setup ready, run 'sudo reboot' to apply all changes"
echo
} 2>&1 | sudo tee $LOG_FILE