# Setup for Raspberry 4 Model B 2Gt

## Components

- Raspberry Pi (tested on 2, 4B)
- MicroSD card + adapter
- card reader to access the sd card on the computer


## Raspberry OS basic setup

### 1. Install the operating system and set up user, Wi-Fi, ssh access

#### With Raspberry Pi Imager

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

#### With some other utility

If you do not use the Raspberry Pi Imager to set up the SD card, the following steps are required:

1. Download the 32-bit / 64-bit Rasbperry Pi OS Lite from here: https://www.raspberrypi.com/software/operating-systems/
2. Flash the image to the SD card with the utility of your choice (options for Mac, Linux, Windows?)
3. Fill in required details in the configuration files in configuration folder and copy them to the boot folder on the SD card (this is the folder that should open when you open the SD card volume on your computer):
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


### 2. Installing needed packages



### 3. Mount SSD



## Real time clock (RTC) module



## Set up recording

### 1. Audio



### 2. GPS



### 3. Water pressure



## Test & run


