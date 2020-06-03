COLOR ?= auto
CARGO = cargo --color $(COLOR)

.PHONY: all build check clean doc fmt release test update

all: build

build:
	$(CARGO) build

check:
	$(CARGO) check

clean:
	$(CARGO) clean

doc:
	$(CARGO) doc

fmt:
	$(CARGO) fmt

release:
	$(CARGO) build --release

test: build
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --workspace
	$(CARGO) test --workspace --lib

update:
	$(CARGO) update
