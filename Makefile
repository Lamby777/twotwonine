.PHONY: all build install uninstall

prefix ?= /usr/local
bindir ?= $(prefix)/bin
datadir ?= /usr/share

all: build

build:
	cargo build --release --locked

install:
	# Install the binary to the specified bin directory
	install -Dm755 target/release/twotwonine $(DESTDIR)$(bindir)/twotwonine

uninstall:
	rm -f $(DESTDIR)$(bindir)/twotwonine
