build: 
	cargo build --release
install:
	cp ./target/release/ppjson /usr/local/bin/