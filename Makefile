CARGO ?= cargo

.PHONY: all build run clean fmt check

all: build

build:
	$(CARGO) build

run:
	$(CARGO) run -- $(ARGS)

clean:
	$(CARGO) clean

fmt:
	$(CARGO) fmt

check:
	$(CARGO) check


