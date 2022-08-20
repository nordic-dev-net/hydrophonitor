#!/bin/sh

# Get soundcard name
soundcard=$(grep USB /proc/asound/cards | grep -oe "\[.*]" | tr -d "[] ")

# Start jack server
/usr/bin/jackd -P75 -d alsa -d hw:${soundcard} -r 44100 -p 512 -n 3 &
