BIN := epoch
SRC := $(shell find src -name '*.rs')

all: $(BIN)

$(BIN): $(SRC)
	cargo build --release
	cp target/release/$@ $@

watch:
	EPOCH_PORT=8000 EPOCH_DB_PATH=test.db RUST_LOG=info cargo watch -x "run -- serve"

clean:
	rm -rf $(BIN)
	rm -f test.db
	cargo clean
