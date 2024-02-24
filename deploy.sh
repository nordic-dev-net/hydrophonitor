#!/usr/bin/env bash

set -x

TARGET_USER="kaskelotti"

# Target IP can be passed as command line argument
DEFAULT_IP="192.168.1.112"
TARGET_IP=${1:-${DEFAULT_IP}}

NIX_SSHOPTS="-o RequestTTY=force" nixos-rebuild switch --flake .#raspberry-pi-4 --target-host ${TARGET_USER}@${TARGET_IP} --use-remote-sudo
