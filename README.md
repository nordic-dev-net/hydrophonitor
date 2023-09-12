# Hydrophonitor

A NixOS flake based operating system for Raspberry Pi 3 (tested on v1.2) and Raspberry Pi 4 (tested on 4B), configured for recording audio through hydrophones via a USB sound card, logging GPS data through a USB GPS dongle, and recording depth through a pressure sensor and an analog-to-digital converter (depth recording not implemented yet).

## Overview

Module           | Description
-----------------|-
audio-recorder     | Receive an audio signal from the DAC and write it on disk in `.wav` format.
gps-recorder      | Record position and time of the device in `.json` format.
*depth-recorder     | Record depth of the device and save it in `.csv` format.
real-time-clock | Configure and use an RTC to update system time from hardware clock when not connected to the internet.
shutdown-button | Gracefully shut down the system using a button attached to the GPIO pin.
*device-controls | Provide device control using physical buttons.

*) todo, not implemented yet

### Data Formats

Type        | Output file format | Output file name                     | Output structure
------------|--------------------|--------------------------------------|------------------
Audio Data  | .wav               | `<start-time-of-recording>.wav`  | Each recorded batch will be written to its own file in `/output/audio` folder
GPS Data    | .json               | `<start-time-of-recording>.json`     | All data written to a single file in `/output/gps`

### Notes on Modules and Configuration Options

- `audio-recorder` module
  - `audio-recorder` systemd service connects to a USB soundcard and starts recording by default in batches to a specified output directory
  - configuration options and default values:
    - `output-folder` (/output/audio)
    - `sample-rate` (192000)
    - `sample-format` (S32_LE)
    - `channels` (4)
    - `max-file-time-secs`(300)
- `gps-recorder` module: fetched from hydrophonitor-gps repository
  - enables gpsd
  - `gps-recorder` systemd service starts `gps-recorder` program that listens for and logs GPS data to a json file in a specified directory
  - configuration options and default values:
    - `output-folder` (/output/gps)
    - `interval-secs` (10)
- `real-time-clock` module
  - `i2c-rtc-start` systemd service configures I2C bus to connect to DS3231 RTC attached to GPIO pins of the RPi and updates system time to hardware clock time
  - configuration options and default values:
    - `i2c-bus` (1)
- `shutdown-button` module (currently only for RPi 4)
  - `shutdown-button` service runs `shutdown-button` program that listens for a button press (by default from GPIO pin 21) and once button press is detected, runs a graceful shutdown
  - configuration options yet to be implemented in the program:
    - `gpio-pin` (21)
    - `shutdown-press-secs` (1)


## Other Configurations

- user `kaskelotti` is created with sudo privileges
- ssh enabled with password authentication
- `i2c-dev`, `i2c_bcm2708`, and `rtc_1307` kernel modules enabled for i2c-rtc, `i2c-tools` package installed
- `deploy-rs` used for deployments (after initial bootstrapping and connecting the pi over wifi or ethernet)

### Enabling WiFi

Add the following to targets/<raspberry-pi-model>/default.nix where "SSID name" is replaced with the network name and "SSID password" is the network password:

```nix
  networking = {
    hostName = "kaskelotti";
    wireless = {
      enable = true;
      networks = {
        "SSID name" = {
          psk = "SSID password";
        };
      };
      interfaces = ["wlan0"];
    };
  };
```

## Building the Image on Mac M1

Image has been built with UTM (virtualization for silicon Macs, now running on 2020 M1 Air) on Ubuntu 22.04 VM (with nix installed) with the following command:
```
nix build .#systems.raspberry-pi-4.config.system.build.sdImage --extra-experimental-features "nix-command flakes"
```
The result image has to be copied from the VM nix store path to the directory that was shared between the client and the host.
```
cp -rL result/sd-image/nixos-sd-image-23.11.20230908.db9208a-aarch64-linux.img .
```

Initially, image is flashed to the SD card with the following command (on the host)
```
diskutil list # check SD card, here /dev/disk4
diskutil unmountdisk /dev/disk4
sudo dd if=../UTM/hydrophonitor/nixos-sd-image-23.11.20230702.0fbe93c-aarch64-linux.img of=/dev/disk4 status=progress
diskutil eject /dev/disk4
```

After bootstrapping and connecting the Pi to the local network, deploy-rs can be used for deployments from UTM (update correct IP address to the deploy configuration):
```
nix run github:serokell/deploy-rs .#raspberry-pi-4
```

## Notes

- On Raspberry Pi 4, we use the [Argon One m.2](https://argon40.com/products/argon-one-m-2-case-for-raspberry-pi-4) case with [Intenso M.2 TOP SATA III 1TB SSD](https://www.intenso.de/en/products/solid-state-drives/M.2-SSD-Top) and boot from SSD. We lacked a USB extension cable that would have allowed us to flash the SD card image directly to the SSD from the computer, so we ended up booting the Pi from a USB stick, copying the SD card image to the system on the USB stick, and from there flashing the image with `dd` to the SSD (which showed up as /dev/sda). After removing the USB stick, the Raspberry Pi successfully booted from the SSD. The setup also works using an ordinary SD card without an SSD. It would also be possible to run the operating system from the SD card, add an additional SSD, partition and format it and configure the outputs to be written to the SSD.
- On Raspberry Pi 3, we only tested with an SD card without the SSD, and witnessed some buffer overruns with `arecord` with the default settings, i.e. some audio output is lost. Also, running the `shutdown-button` program on Raspberry Pi 3 failed with permission denied on /dev/mem, even though the service is run as root.

