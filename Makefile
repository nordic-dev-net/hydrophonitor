install:
	cd scripts && setup-raspberry-pi.sh

build-audio:
	cd audio-logger && cargo build --release

start-all:
	cd scripts && start-all.sh
