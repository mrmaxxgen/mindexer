install:
	cargo build --release
	mv target/release/mindexer /usr/local/bin/mindexer