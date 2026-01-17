BIN := epoch
SRC := $(shell find src -name '*.rs')

all: $(BIN)

$(BIN): $(SRC)
	cargo build --release
	cp target/release/$@ $@

watch:
	RUST_LOG=info cargo watch -x "run -- serve"

clean:
	rm -rf $(BIN)
	cargo clean
