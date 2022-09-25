all:
	cargo build --release

test: all
	mkdir -p recordings
	bash run_test.sh

clean:
	cargo clean

fclean: clean
	rm -rf recordings

re: fclean
	cargo build --release

.PHONY: all clean fclean re test
