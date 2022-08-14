# Raspberry pi initial setup

## Outline

This document will shed light on the way to setting up a Raspberry Pi computer to be used as a data collector in a drifting buoy hydrophone setup. 

## SD-card preparation

We want to use the Solo Field Recorder OS SoloOS on th RasPi and in order to do that we must prepare a SD-card so that it can be flashed with aforementioned OS.

On most Windows operating systems one can format the SD-card by by plugging the card to the PC with a card-reader and right-clicking on the thumbnail of the drive and selecting format from the drop-down menu. 

For linux users the sudo dd command, that is used when flashing the sd-card will owerwrite any data on the card, so there is no need to format the sd-card in advance.

## Flashing the SOLO OS

The documentation at: https://solo-system.github.io/documentation/flashing.html is quite extensive and it explains te steps needed to get the SoloOS flashed onto an sd-card. SoloOS is a Operating system that is based on RaspberryPiOS and it can be downloaded from the website in the above link. Instead of using the Win32DiskImager program, one can use Balena Etcher to flash the card.

In order to flash the sd-card on a linux machine, use the command: lsblk to locate where your sd-card is mounted. The size od the sd-card is usually a good indicator of which drive the sd-card is. On a debian machine the drive usually mounts to /dev/sdb. Use the command sudo dd if=sosi-2019-09-20.img of=/dev/sdX bs=4M conv=fsync Make sure you replace the img. File with the file name of the file you have downloaded and substitute dev/sdX with the appropriate drive that your USB-stick is mounted on. Note that a wrong letter here can result in loss of your whole data, so be very careful when writing the command. Also, before running the command, remember to unmount the drive you want to write on using the umount dev/sdX command.

## Re-partitioning

Setting up the Solo for this project will require updating the Solo, which is not possible at the outset, because the Solo is configured to give maximum amount of storage capacity on the sd-card and therefore there is very little space left on the boot partition for updating the device. Thus, before proceeding to the next phase we must re-partition the sd-card to allow for installing additional software on the Pi.

I managed to do the repartitioning following this post: https://moreless.medium.com/extend-partition-and-file-system-on-raspberr-a48af9e90858

The Solo OS is set up so that it automatically creates a third partition for the data to be stored on the sdcard. This is done by a bash script on startup. This will leave the rootfs partition wedged between the boot partition and the main partition making it very difficult to expand it. Thus I opted to use the fdisk tool to expand the rootfs partition before the 3rd partition is created. The way to do this is discussed in appendix 2

## Run fdisk

In the linux terminal, with the sdcard connected to your Debian 10 computer, locate the sdcard.
	
	lsblk

This will give you the location. Usually it’s /dev/sdb

First you must unmount the drive;
	
	sudo umount /dev/sdX #where X is substituted for your drive letter

	sudo fdisk /dev/sdX 	#where X is substituted for your drive letter

	Command: p to see the start and end of the partitions

	F to see the free space that is unallocated

	d to delete the partition 2
	
Scroll up for reference, because you must start the new partition on exactly the same sector that it was on or the partition will not work.

	n for new partition, make it a primary partition number 2

First sector, use same as the previous partition 2. End sector, grow this an appropriate amount. I substituted the first number by the number 9, which gave me ample space, around 4 gib.

	N as in do not remove the signature.

	p to view the newly created partitioning

	w  to write the new partition

## Run fsck

Now it is a good idea to check the filesystem.

	sudo e2fsck -f /dev/sdX2	#again substituting the X

	sudo resize2fs /dev/sdX2

	

## Setting up Wifi

In order to set up the Raspberry to connect to Wifi automatically we need to make a file called wpa_supplicant.conf in your equivalent of /home/media/username/boot.

To do this you can insert the sd card with the flashed solo image into your pc and using a terminal window to make the file. Navigate to the boot folder of your sd-card, then

	sudo nano wpa_supplicant.conf

The file is a text file that needs to have the the text:

	ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev
	update_config=1
	country=FI
	network={
	 ssid="Name_of_network"
	 psk="xxxx"
	 }

	Where ssid = the name of the network you want to connect to the pi via and psk stands for passkey.

## Enabling SSH

To enable SSH you need to make an empty file in the boot partition:

	sudo touch ssh

## Installing pytho



## Configuring the SOLO

Configuration of the Solo is done by editing the different conf files, namely the solo.conf and amon.conf files. You can alter these files in Windows by connecting the sd-card to your PC and navigating to the rootfs  or boot respectively and editing the files with your preferred text editor, such as Notepad. Alternatively one can SSH into the RasPi and use nano  or vim to edit the files.

A noteworthy thing I came across is that there are two files named amon.conf in the Solo filesystems. One is situated in the rootfs behind /home/amon/amon/ the other one is in the boot folder. Some of the settings in the amon.conf files are overlapping and the boot amon.conf file overrides the rootfs amon.conf.

For claritys sake, what I did to the configurations:

	Boot amon.conf : 

	SAMPLERATE=”-r192000”	#I changed the samplerate to 192000kHz
	CHANNELS=”-c2”	#Set the channels option for “2”

	Rootfs amon.conf :

	SAMPLERATE=”-r192000” #To be sure
	CHANNELS=”-c2”
	DURATION=1 		#I set it at one minute to see that the solo records 		#several files. Later I will set it to 60 as in 60 minutes
	AMONDATA=”/mnt/sdcard/amondata” 	# I exchange the sdcard to myhdd after 			setting up the external HDD 
	
The configuration is also well documented at : https://solo-system.github.io/documentation/configuration.html What is most of interest to us is the self-explanatory; SAMPLERATE=”-r16000” controls the sample rate of the audio in Hz. Set it to 8000, 16000, 44100 or anything up to 192000, depending on your audio-card’s capability. Also in the amon.conf file one ought to change the value of DURATION=10 to 60 as in 60 minutes. The way to edit the file with a Windows operating system is to right click on the notepad program → Run as administrator → Allow system to make changes, Yes. Then one ought to change the afore mentioned values and save the file.

## First recording

The SOLO will automatically start recording when the power cord is attached to the Pi. Make sure the Sound card is attached to the Pi via USB and that the power indicator light tuns on on the Sound Card and that the Microphone is plugged in. The Red LED on the Pi indicates that it is getting power and the green LED indicates that the software is alive with a hearbeat-like pulsation.

## Extracting the data

The wav. Files can be extracted from the Pi by removing the SD-card and plugging it to the computer via card reader.Once the card is connected, the PC will recognise the card, and the audio will become available to copy. You should ensure that you have sufficient storage on your PC to handle all the audio data files (Solos can collect a LOT of data). If you are a Windows user, the PC might not recognize the sd-card. If this is the case you need to install Linux_reader.

On the solo site it says: “Once you have safely copied the data, you should consider the contents of the memory card to be useless, reflash the card. Note: Deleting the audio and re-using the card in a Solo without reflashing - might work, but is not recommended. Always use a freshly flashed memory card when you prepare a Solo.”

This is all well and good but one should bear in mind that if you flash the sdcard all your conf. file settings are done away with and you must set them again. Hence, I will try to get away with erasing the data and re-using the solo setup I have and only if trouble arises will I re-flash the sd-card. If one uses the external HDD instead this problem might be a smaller one, maybe.

## Setting up SSD

If one wants to store files on an external HDD one must mount the HDD and alter the path to the wav files of the recorder. There are at least two ways of doing this. One way to do it is to SSH into the Pi using a network cable and a home router. Another way is to alter the solo.conf file in the same manner as noted earlier in this document. The thing to alter is to uncomment SOLO_POWERMODE=normal and comment SOLO_POWERMODE=lowest. To comment these lines one is to use the # symbol. This will disable the power saving feature of turning off the HDMI-port power.  The next step is to connect a display to the HDMI port and a keyboard to one of the the usb-slots. With the SD-card back in the Raspberry it can now be turned on by connecting power to the Pi. Furthermore, you can connect the sd-card to the pc via card reader and alter the solo.conf file that way.

Configuring the SSD	A more extensive view on how to configure external drives on the Raspberry can be found at https://www.raspberrypi.com/documentation/computers/configuration.html#external-storage-configuration

Note that the Raspberry Pi OS which the Solo OS is based upon is built on Debian Linux, so navigating the filesystem with the terminal is done with linux commands. This is where your Linux terminal navigation skills come in handy. The procedures are the same whether you are SSH:ing into the pi or using a monitor and keyboard to do the job.

Upon startup the Pi asks for the login username and password. The defaults are pi as username and raspberry password. You can connect your external hard disk, SSD, or USB stick to any of the USB ports on the Raspberry Pi, and mount the file system to access the data stored on it. By default, your Raspberry Pi automatically mounts some of the popular file systems such as FAT, NTFS, and HFS+ at the /media/pi/<HARD-DRIVE-LABEL> location.

	Use the command: s
	sudo lsblk -o UUID,NAME,FSTYPE,SIZE,MOUNTPOINT,LABEL,MODEL
	to list the mounted usb devices on the raspberry.
	
	Use the SIZE, LABEL and MODEL columns to identify the name of the disk partition that points to your storage device. For example sda1

	

## Configuring the SSD

The FSTYPE column contains the filesystem type. If your storage device uses an
	exFAT file system, install the exFAT driver:

	sudo apt update
	sudo apt install exfat–fuse

If your storage device uses an NTFS file system, you will have read-only access to it.
If you want to write to the device, you can install the ntfs-3g driver:

	sudo apt update
	sudo apt install ntfs-3g


Run the following command to get the location of the disk partition(example, sda1):

	sudo blkid

Create a target folder to be the mount point of the storage device. The mount point name used in this case is myhdd. You can specify a name of your choice:

	sudo mkdir /mnt/myssd

Mount the storage device at the mount point you created(substitute X with your target letter):

	sudo mount /dev/sdX  /mnt/myssd	

Verify that the storage device is mounted successfully by listing the contents:

	ls /mnt/myssd

When in the myhdd directory one should make the directory to put the wavs in using the command sudo mkdir amondata .

Also, we want to make a folder here for the gps data.  sudo mkdir gpsdata

It is then possible to make the average user able to access the data, (this is optional).

	sudo chmod 777 /mnt/myssd
	sudo chmod 777 /mnt/myssd/amondata
	sudo chmod 777 /mnt/myssd/gpsdata

After completing the previous step, amon.conf(located in the rootfs directory) file must be altered again to accommodate the new HDD. Write the line AMONDATA=”/mnt/myssd/amondata” Also, to save power one should revert the solo.conf file back to read  SOLO_POWERMODE=lowest. And Bob’s your uncle. Now the Raspberry should, in theory, be ready to record to the HDD.

## Automatic mounting

The way to automatically mount an external drive is discussed in detail in https://www.raspberrypi.com/documentation/computers/configuration.html#external-storage-configuration


	 
## Configuring the GPS

We stand on the shoulders of giants, so, here’s another link of instructions I followed to set up the GPS. https://www.instructables.com/Raspberry-Pi-3-GPS-Data-Logger/ 

## Logging the data

Using OZZmakers script, that is tweaked to fit our needs one can save the GPS data in a csv file. 



## Launching logger script

I made a shell script that I cobbled together from different sources online. 

	sudo nano ~/GPSlogger.sh

	#!/bin/bash

	cd /
	cd home/pythonscripts
	sudo python gpsscript.py

Here, gpsscript.py is exactly a copy of OZZmakers GPS logger script.

The shell script I made must be made executable;

	sudo chmod 755 GPSlogger.sh

Then we will alter the crontab to invoke running the shell script on startup.

	crontab -e
 	
Add the below line to the bottom

	@reboot /home/pi/GPSlogger.sh



