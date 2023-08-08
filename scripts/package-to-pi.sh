#!/bin/bash

set -e

run_id=$(gh run list -w "Build package" | grep "completed" | head -n 1 | rev | cut -f 3 | rev)

gh run download $run_id -n built-package

user="pi"
pi_ip="192.168.1.108"

scp hydrophonitor.tar.gz $user@$pi_ip:/home/$user

ssh $user@$pi_ip <<EOF
	cd /home/$user
	tar -xvf hydrophonitor.tar.gz
	cd hydrophonitor
	./scripts/install-from-package.sh
EOF
