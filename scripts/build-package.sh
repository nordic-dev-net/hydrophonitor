#!/bin/bash

set -xe

PACKAGE_DIR=package

mkdir -p ${PACKAGE_DIR}

# Build audio-logger binary and copy to package directory

# Check if Rust toolchain is installed, if not ask if it should be installed
if ! command -v rustup &> /dev/null; then
  if [ "$1" = "--install-rust" ]; then
	sudo apt-get update && sudo apt-get install -y build-essential curl git
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	source "$HOME/.cargo/env"
  else
	echo "Rust toolchain is required to build the audio-logger binary"
	echo "To install the Rust toolchain, run this script with the --install-rust flag"
	exit 1
  fi
fi

sudo apt-get update && sudo apt-get install -y libasound2-dev libjack-dev

# # Check if docker is installed, if not ask if it should be installed
# if ! command -v docker &> /dev/null; then
#   echo "Docker is not installed, do you want to install it? [y/n]"
#   read -r install_docker
#   if [ "$install_docker" = "y" ]; then
# 	curl -fsSL https://get.docker.com -o get-docker.sh
#   else
# 	echo "Docker is required to build the audio-logger binary"
# 	exit 1
#   fi
# fi

# rustup target add aarch64-unknown-linux-gnu
# cargo install cross --git https://github.com/cross-rs/cross

# cross build --release --target=aarch64-unknown-linux-gnu
# cp target/aarch64-unknown-linux-gnu/release/audio ${PACKAGE_DIR}

cargo build --release
cp target/release/audio ${PACKAGE_DIR}

# Copy depth-logger files to package directory
cp -r depth-logger ${PACKAGE_DIR}

# Copy gps-logger files to package directory
cp -r gps-logger ${PACKAGE_DIR}

# Copy scripts to package directory
cp -r scripts ${PACKAGE_DIR}

# Copy config file to package directory
cp hydrophonitor-config.txt ${PACKAGE_DIR}
echo "Package created in ${PACKAGE_DIR}"

# Create a tar gzipped archive of the package directory
cd ${PACKAGE_DIR} && tar -czf hydrophonitor.tar.gz *
echo "Package created in ${PACKAGE_DIR}/hydrophonitor.tar.gz"
