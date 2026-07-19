install:
	cargo build --release
	mv target/release/mindexer /usr/local/bin/mindexer
	mkdir /etc/mindexer
	touch /etc/mindexer/config.txt /etc/mindexer/logs.txt
uninstall:
	rm /usr/local/bin/mindexer
removeconfig:
	rm /etc/mindexer/config.txt
removelogs:
	rm /etc/mindexer/logs.txt