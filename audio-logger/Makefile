all:
	cargo build --release

test: all
	mkdir -p recordings
	bash run_test.sh

clean:
	cargo clean

fclean: clean
	rm -rf recordings

re: fclean all

.PHONY: all clean fclean re test
