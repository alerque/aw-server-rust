.PHONY: all build aw-server-rust aw-webui install install-usr install-opt package service-symlink-opt service-symlink-usr

all: aw-server-rust
build: aw-server-rust

DESTDIR ?=
PREFIX ?= /usr/local
SERVICE_OPT=$(DESTDIR)/opt/activitywatch/aw_server_rust/aw-server-rust.service
SERVICE_USR=$(DESTDIR)$(PREFIX)/share/activitywatch/aw_server_rust/aw-server-rust.service

aw-server-rust:
	cargo build --release

aw-webui:
	make -C ./aw-webui build

package: aw-server-rust aw-webui
	# Clean and prepare target/package folder
	rm -rf target/package
	mkdir -p target/package
	# Copy binary
	cp target/release/aw-server-rust target/package
	# Copy webui assets
	mkdir -p target/package/aw_server_rust
	cp -rf aw-webui/dist target/package/aw_server_rust/static
	# Copy service file
	cp -f aw-server-rust.service target/package/aw_server_rust

install: install-opt

service-symlink: service-symlink-opt

install-opt: target/package
	mkdir -p $(DESTDIR)/opt/activitywatch
	cp -rf target/package/* $(DESTDIR)/opt/activitywatch
	mkdir -p $(DESTDIR)$(PREFIX)/bin/
	ln -sf /opt/activitywatch/aw-server-rust $(DESTDIR)$(PREFIX)/bin/aw-server-rust

service-symlink-opt:
	ln -sf $(SERVICE_OPT) ~/.config/systemd/user/aw-server-rust.service

install-usr:
	mkdir -p $(DESTDIR)$(PREFIX)/share/activitywatch
	cp -rf target/package/* $(DESTDIR)$(PREFIX)/share/activitywatch
	mkdir -p $(DESTDIR)$(PREFIX)/bin/
	ln -sf $(PREFIX)/share/activitywatch/aw-server-rust $(DESTDIR)$(PREFIX)/bin/aw-server-rust

service-symlink-usr:
	ln -sf $(SERVICE_USR) ~/.config/systemd/user/aw-server-rust.service
