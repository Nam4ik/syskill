CARGO ?= cargo
CC ?= gcc


.PHONY: all build run clean fmt check install



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

kmod: 
	make -f src/non_critical/kern_panic/Makefile build 

install:
	mv target/debug/suicidekit /usr/local/bin/suicidekit
	make -f src/non_critical/kern_panic/Makefile install