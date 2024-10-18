# Docker Registry Actions

Docker Registry Actions is a service that monitors Docker registry images and automatically restarts containers from a specified Docker Compose configuration when the images are updated.

## Usage

Run `docker-registry-actions -h` to see all available configuration options.

## Installation

### Ubuntu

To install Docker Registry Actions on Ubuntu and set it up as a `systemd` service, use the following command:

```bash
bash <(curl -s https://raw.githubusercontent.com/simotasca/docker-registry-actions/master/installers/linux/install.sh)
```

To stop the service and uninstall the program:

```bash
bash <(curl -s https://raw.githubusercontent.com/simotasca/docker-registry-actions/master/installers/linux/uninstall.sh)
```

### Building from Source

Simply: `cargo build --release`
