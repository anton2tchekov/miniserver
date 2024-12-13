all: build doc copy
all-dev: build-dev doc copy-dev

build:
	cargo build --release

build-dev:
	cargo build

doc:
	cargo doc

copy:
	cp target/release/miniserver .

copy-dev:
	cp target/debug/miniserver .

clean:
	cargo clean
	rm miniserver