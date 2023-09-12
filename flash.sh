# Flash image to sd-card
#
# Check sd-card device name with lsblk

SDCARD=$1
IMAGE=$2

sudo dd if=${IMAGE} of=/dev/${SDCARD} status=progress 
