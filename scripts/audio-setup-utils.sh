#!/usr/bin/env sh

# Install utils for cpu freq
sudo apt-get install cpufrequtils
sudo cpufreq-set -r -g performance
sudo echo "ENABLE="true"
GOVERNOR="performance"
MAX_SPEED="0"
MIN_SPEED="0" " | sudo tee -a /etc/default/cpufrequtils

# Set CPU governor
sudo sed -i 's/exit 0/sudo cpufreq-set -r -g performance/g' /etc/rc.local
sudo echo "exit 0" | sudo tee -a /etc/rc.local

# Set realtime priority and memlock
sudo echo "
@audio nice -15 
@audio - rtprio 90       # maximum realtime priority
@audio - memlock unlimited  # maximum locked-in-memory address space (KB)
" | sudo tee -a /etc/security/limits.conf

# Set swappiness
# This setting changes the so-called swappiness of your system, 
# or in other words, the moment when your system starts to use its swap partition. 
sudo echo "
vm.swappiness = 10
fs.inotify.max_user_watches = 524288
" | sudo tee /etc/sysctl.conf
