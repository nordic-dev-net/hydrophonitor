# Setup for Raspberry 4 Model B 2Gt

## Components

- Raspberry Pi 4B
- One of the following, depending on whether an SD card or an SSD is used:
  - 1) MicroSD card + adapter & card reader to access the sd card on the computer
  - 2) USB SSD drive OR Argon One M.2 case + compatible M.2 SATA SSD (Key B or Key B&M), we used Intenso M.2 SSD TOP SATA III 1TB
- Audio card connected via USB and microphone/hydrophone(s) attached to the audio card
- USB GPS receiver
- Depth recording components:
  - Pressure sensor
  - Adafruit ADS1015 ADC
  - breadboard, resistors, jumper wires, 12V battery
- RTC module (DS3231)


## First time setup

TODO

### 1. Host environment

TODO

### 2. Define custom configuration

TODO

### 3. Build first SD image

TODO

### 4. Flash the image to the SD card or SSD

### 5. Connect to the Pi over SSH

#### Plug the SD card in and connect to the Raspberry Pi over SSH

Plug the SD card in the Raspberry Pi. Connect the audio card and the GPS receiver over USB to the Raspberry Pi, and plug the power cable. It will take some time for the Raspberry Pi to be ready to accept SSH connections.

To figure out what IP address the Raspberry Pi has been assigned in the local network, a tool called `nmap` is needed.

To check whether nmap is already installed on the system, open a terminal and run the following command (write it to the terminal and press Enter):

```
nmap --version
```

If this prints out version information about nmap (e.g. Nmap version 7.93 ( https://nmap.org)), it is installed. Otherwise, installation instructions can be found here: https://nmap.org/download.html

After installing nmap, we first need to determine the subnet in which we will search for the Raspberry Pi IP address. This can be done by first determining our host device's IP address:

```
ip a | grep 192.168
```

This should print out something similar to
```
❯ ip a | grep 192.168
    inet 192.168.1.117/24 brd 192.168.1.255 scope global dynamic noprefixroute enp3s0
```

Here, 192.168.1.117 is our own IP address. From this, we can deduce our subnet by taking the first three parts of the IP address and replacing the last part with zero, and including the subnet mask (24): `192.168.1.0/24`

Then, run the following command (it will ask for your user password, write it and press Enter) to find all devices connected to the local network, and replace `192.168.1.0/24` with the version with your own IP address:

```
sudo nmap -sn 192.168.1.0/24
```

The result will contain a series of discovered devices (hosts) with the following information for each device:

```
Nmap scan report for 192.168.1.108
Host is up (0.18s latency).
MAC Address: E4:5F:01:B3:65:DE (Raspberry Pi Trading)
```

The Raspberry Pi should show up with its IP address (here, 192.168.1.108), MAC address and a name after the MAC address that should help identifying it (here, it's Raspberry Pi Trading).

Now, this IP address can be used to connect to the Raspberry Pi over SSH on the command line. Connect by running the command `ssh <user>@<IP address>`, which with a user called `kaskelotti` and an IP address of 192.168.1.108 would be

```
ssh kaskelotti@192.168.1.108
```

When asked `Are you sure you want to continue connecting (yes/no/[fingerprint])?`, type `yes` and press Enter. Then, write the Raspberry Pi user's password when asked and press Enter.

After successfully connecting, your prompt should change to `<user>@<hostname>:~` or something similar.

### 6. Verifying that services are running as expected

TODO

## Updates & deployments after first time setup

TODO

### Making changes

TODO

### Deploying changes to the Raspberry Pi

TODO
