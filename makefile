.PHONY: *

dev:
	cargo run

release:
	cargo build --release

install: release uninstall
	sudo ln target/release/docker-registry-actions /usr/bin

uninstall:
	sudo rm -f /usr/bin/docker-registry-actions ||: