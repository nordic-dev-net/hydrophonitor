#!/bin/bash

set -ex

echo "Setting up audio recording"

# Install packages
sudo apt-get update && sudo apt-get install -y libasound2-dev libjack-dev

# Get ID and number of the USB audio device
card_number=$(aplay -l | grep -i usb | grep -i audio | cut -d ' ' -f 2 | cut -d ':' -f 1)

# Change default audio device
sudo touch /etc/asound.conf

config="pcm.!default {
  type plug
  slave {
    pcm \"hw:$card_number,0\"
  }
}

ctl.!default {
    type hw
    card $card_number
}"

if ! grep -q "$config" /etc/asound.conf; then
  echo "$config" | sudo tee -a /etc/asound.conf
fi


cd $HOME/hydrophonitor/audio-logger && cargo build --release
