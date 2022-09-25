all:
	cargo build --release

test:
	cargo build --release
	mkdir -p recordings
	bash run_test.sh

clean:
	cargo clean

fclean: clean
	rm -rf recordings

re: fclean
	cargo build --release

.PHONY: all clean fclean re test
