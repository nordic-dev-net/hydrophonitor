# Setup for Raspberry 4 Model B 2Gt

## Components

- Raspberry Pi (tested on 4B)
- MicroSD card + adapter
- Card reader to access the sd card on the computer
- Audio card connected via USB and a microphone/hydrophone attached to the audio card
- USB GPS receiver
- Depth recording components:
  - Pressure sensor
  - Adafruit ADS1015 ADC
  - breadboard, resistors, jumper wires, 12V battery
- RTC module (DS3231)


## Raspberry OS basic setup

### 1. Install the operating system and set up user, Wi-Fi, ssh access

#### 1.1 With Raspberry Pi Imager

The easiest way to install the operating system (Raspberry Pi OS, a Linux Debian-based OS) is to use the official Raspberry Pi Imager utility which works on macOS, Ubuntu and Windows.

Install from here: https://www.raspberrypi.com/software/

After installing, plug the SD card to the computer and launch Raspberry Pi Imager.

Then the following steps:

1. Select operating system: click Raspberry Pi OS (other) and then, depending on the Pi, either a 32-bit or 64-bit Raspberry Pi OS Lite
2. Select storage: the sd card should be listed
3. Click the cog icon to set some configurations:
	- Enable SSH (use password authentication)
	- Set username and password
	- Configure wireless LAN: input the name and password of the wi-fi network, select Wireless LAN country
	- Set locale settings: select options correct for you
4. Click Write (all existing data on the SD card will be erased and the OS installed)

#### 1.2 With some other utility

If you do not use the Raspberry Pi Imager to set up the SD card, the following steps are required:

1. Download the 32-bit / 64-bit Rasbperry Pi OS Lite from here: https://www.raspberrypi.com/software/operating-systems/
2. Flash the image to the SD card with the utility of your choice (options here for Mac, Linux, Windows?)
3. Fill in required details in the configuration files in `hydrophonitor/pi-config` folder and copy them to the `boot` folder on the SD card (this is the folder that should open when you open the SD card volume on your computer):
	- ssh.txt: this enables ssh on the Raspberry Pi, no need to edit the file (it's empty, the existence of the file in the boot folder is enough)
	- userconf.txt: creates a user
    	- replace <username> with the username of choice (e.g. pi)
    	- replace <encrypted password> with an encrypted version of your password which can be created with the openssl command line tool:
        	- open Terminal, write `openssl passwd` and press Enter
        	- input your password and press enter (asked twice)
        	- as output, you will get the encrypted version of the password
  	- wpa_supplicant.conf: set up Wi-Fi
    	- replace <Insert 2 letter ISO 3166-1 country code here> with your country code (e.g. FI)
    	- replace "<Name of your wireless LAN>" with the name of your Wi-Fi network, e.g. "explorersden"
    	- replace "<Password for your wireless LAN>" with the Wi-Fi password, e.g. "password"


### 2. Setting up the recording programs on the Raspberry Pi

After flashing the operating system to the SD card, it should show up as volume called `boot`.

To install all the needed components and to configure the Raspberry Pi to start the recordings when it is turned on, four steps are needed: copying the needed files to the SD card, putting the SD card in the Raspberry Pi and connecting to it on the command line over SSH, running an installer script on the command line, and finally letting it restart and verify that everything works as intended.

#### 2.1 Copy files to the SD card, set configuration values

First, set the configuration values in the file `hydrophonitor/hydrophonitor-config.txt`. Then, copy the entire `hydrophonitor` folder to the SD card (simple Ctrl+C and Ctrl+V works). 

#### 2.2 Plug the SD card in and connect to the Raspberry Pi over SSH

Plug the SD card in the Raspberry Pi. Connect the audio card and the GPS receiver over USB to the Raspberry Pi, and plug the power cable. It will take some time for the Raspberry Pi to be ready to accept SSH connections.

To figure out what IP address the Raspberry Pi has been assigned in the local network, a tool called `nmap` is needed.

To check whether nmap is already installed on the system, open a terminal and run the following command (write it to the terminal and press Enter):

```
nmap --version
```

If this prints out version information about nmap (e.g. Nmap version 7.93 ( https://nmap.org)), it is installed. Otherwise, installation instructions can be found here: https://nmap.org/download.html

After installing, run the following command (it will ask for your user password, write it and press Enter) to find all devices connected to the local network:

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

Now, this IP address can be used to connect to the Raspberry Pi over SSH on the command line. Connect by running the command `ssh <user>@<IP address>`, which with a user called `pi` and an IP address of 192.168.1.108 would be

```
ssh pi@192.168.1.108
```

When asked `Are you sure you want to continue connecting (yes/no/[fingerprint])?`, type `yes` and press Enter. Then, write the Raspberry Pi user's password when asked and press Enter.

After successfully connecting, your prompt should change to `<user>@raspberrypi:~` or something similar.

#### 2.3 Run the installer script

After establishing the SSH connection to the Raspberry Pi, change the current directory to the location of the installer script and run it:

```
cd /boot/hydrophonitor/scripts
./setup-raspberry-pi.sh
```

At the end of successful configuration, the script should print "### Setup ready, run 'sudo reboot' to apply all changes". Run the command and input the Raspberry Pi user's password if requested:

```
sudo reboot
```

This will restart the Raspberry Pi and apply the changes made in the setup. On startup, it should now start recording audio, GPS and depth data.

### 3. Set up the real time clock module

There are two scripts that configure the Raspberry Pi to read its system time from the RTC module.

The first part enables the i2c interface (bus 3 with SDA at GPIO pin 23 and SCL at GPIO pin 24) and loads the needed hardware modules at boot. After that, a reboot is needed to enable the hardware interface. The second part updates the hardware clock module time and configures the Raspberry Pi to set the hardware clock time as the system time on startup.

Connect to the Raspberry Pi over ssh, navigate to the home directory, and run the script with following commands:

```
ssh <username>@<IP>
cd $HOME
./hydrophonitor/scripts/setup-rtc-1.sh
sudo reboot
```

```
ssh <username>@<IP>
cd $HOME
./hydrophonitor/scripts/setup-rtc-2.sh
```

### 4. Configuration options

todo

### 5. Mount SSD

todo

## Test & run

todo
