.PHONY: *

dev:
	cargo run -- -c ./config.demo.yml

release:
	cargo build --release
	rm -f installers/linux/docker-registry-actions
	cp target/release/docker-registry-actions installers/linux

install: release uninstall
	sudo ln target/release/docker-registry-actions /usr/bin

uninstall:
	sudo rm -f /usr/bin/docker-registry-actions ||: