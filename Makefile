BIN := target/releases/jlaunch


all: $(BIN)

$(BIN):
	cargo build --release
