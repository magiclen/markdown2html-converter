EXECUTABLE_NAME := markdown2html-converter

all: ./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME)

./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME): $(shell find . -type f -iname '*.rs' -o -name 'Cargo.toml' | grep -v ./target | sed 's/ /\\ /g') $(shell find ./resources -type f | sed 's/ /\\ /g')
	cargo build --release --target x86_64-unknown-linux-musl
	strip ./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME)
	
install:
	$(MAKE)
	sudo cp ./target/x86_64-unknown-linux-musl/release/$(EXECUTABLE_NAME) /usr/local/bin/$(EXECUTABLE_NAME)
	sudo chown root: /usr/local/bin/$(EXECUTABLE_NAME)
	sudo chmod 0755 /usr/local/bin/$(EXECUTABLE_NAME)

uninstall:
	sudo rm /usr/local/bin/$(EXECUTABLE_NAME)

test:
	cargo test --verbose

clean:
	cargo clean
