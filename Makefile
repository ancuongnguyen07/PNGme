build:
	cargo build

test: build
	cargo test
	./test_scripts/cmd_test.sh
