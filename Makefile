all: ./target/x86_64-unknown-linux-musl/release/markdown2html-converter

./target/x86_64-unknown-linux-musl/release/markdown2html-converter: $(shell find . -type f -iname '*.rs' -o -name 'Cargo.toml' | sed 's/ /\\ /g') $(shell find ./resources -type f | sed 's/ /\\ /g')
	cargo build --release --target x86_64-unknown-linux-musl
	strip ./target/x86_64-unknown-linux-musl/release/markdown2html-converter
	
install:
	$(MAKE)
	sudo cp ./target/x86_64-unknown-linux-musl/release/markdown2html-converter /usr/local/bin/markdown2html-converter
	sudo chown root: /usr/local/bin/markdown2html-converter
	sudo chmod 0755 /usr/local/bin/markdown2html-converter

uninstall:
	sudo rm /usr/local/bin/markdown2html-converter

test:
	cargo test --verbose

clean:
	cargo clean
