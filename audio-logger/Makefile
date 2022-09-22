test: all
	bash run_test.sh

all:
	cargo build --release
	mkdir -p recordings

clean:
	cargo clean

fclean: clean
	rm -rf recordings

re: fclean
	cargo build --release

.PHONY: all clean re test