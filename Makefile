build: 
	cargo build --release
uninstall:
	rm /usr/local/bin/ppjson
install:
	cp ./target/release/ppjson /usr/local/bin/