build-audio:
	cd audio-logger && cargo build --release

build-package:
	./scripts/build-package.sh

start-all:
	cd scripts && start-all.sh

clean:
	cargo clean
	rm -rf package