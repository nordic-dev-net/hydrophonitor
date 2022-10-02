#!/bin/sh

# Get ID and number of the USB audio device

card_number=$(aplay -l | grep -i usb | grep -i audio | cut -d ' ' -f 2 | cut -d ':' -f 1)

# Change default audio device

sudo touch /etc/asound.conf

sudo cat << EOF | sudo tee -a /etc/asound.conf
pcm.!default {
  type plug
  slave {
    pcm "hw:$card_number,0"
  }
}

ctl.!default {
    type hw
    card $card_number
}
EOF
